use std::convert::{Into, TryFrom};
use std::str::FromStr;
use std::result::Result as StdResult;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::marker::PhantomData;

use url::Url as StdUrl;
use serde::de::{Deserialize, Deserializer, Visitor, SeqAccess, MapAccess, Error as SerdeError};
use serde::de::value::{MapAccessDeserializer, SeqAccessDeserializer};
use serde_json as json;

// ----------------------------------------------------------------

pub type UnixTimestamp = i64;

// ----------------------------------------------------------------

// https://github.com/serde-rs/json/issues/373
pub fn str_to_option<'de, T, D>(deserializer: D) -> StdResult<Option<T>, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let res: StdResult<json::Value, _> = Deserialize::deserialize(deserializer);
    if let Ok(json::Value::String(s)) = res {
        let i = T::from_str(&s).map_err(SerdeError::custom)?;
        return Ok(Some(i));
    }
    Ok(None)
}

// ----------------------------------------------------------------

pub fn str_to_val<'de, T, D>(deserializer: D) -> StdResult<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(SerdeError::custom)
}

// ----------------------------------------------------------------

pub fn str_to_variant<'de, E, D>(deserializer: D) -> StdResult<E, D::Error>
where
    E: TryFrom<u32>,
    E::Error: Display,
    D: Deserializer<'de>,
{
    let uint: u32 = String::deserialize(deserializer)?.parse().map_err(SerdeError::custom)?;
    TryFrom::try_from(uint).map_err(SerdeError::custom)
}

// ----------------------------------------------------------------

pub trait Underlying<'de> {
    type Struct: Deserialize<'de>;
    type Arr: Deserialize<'de>;

    fn from_arr(t: Self::Arr) ->Self;
    fn from_val(t: Self::Struct) -> Self;
}

#[derive(Deserialize, Debug)]
pub struct VecOrStruct<'dt, T: 'dt>(Vec<T>, PhantomData<&'dt T>);

impl<'de, T: Deserialize<'de>> Underlying<'de> for VecOrStruct<'de, T> {
    type Struct = T;
    type Arr = Vec<T>;

    fn from_arr(t: Vec<T>) -> VecOrStruct<'de, T> {
        VecOrStruct(t, PhantomData)
    }

    fn from_val(t: T) -> VecOrStruct<'de, T> {
        VecOrStruct(vec!(t), PhantomData)
    }
}

pub fn vec_or_struct<'de, T, D>(deserializer: D) -> StdResult<T, D::Error>
where
    T: Deserialize<'de> + Debug + Underlying<'de>,
    D: Deserializer<'de>,
{
    struct VecOrStructVisitor<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for VecOrStructVisitor<T>
        where T: Deserialize<'de> + Underlying<'de>
    {
        type Value = T;

        fn expecting(&self, f: &mut Formatter) -> FmtResult {
            f.write_str("array or map")
        }

        fn visit_map<M>(self, map: M) -> StdResult<T, M::Error>
            where M: MapAccess<'de>
        {
            let t = T::Struct::deserialize(MapAccessDeserializer::new(map))?;
            Ok(T::from_val(t))
        }

        fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where A: SeqAccess<'de>
        {
            let t = T::Arr::deserialize(SeqAccessDeserializer::new(seq))?;
            Ok(T::from_arr(t))
        }
    }

    deserializer.deserialize_any(VecOrStructVisitor(PhantomData))
}

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Url<'dt>(&'dt str);

impl<'de> Into<StdUrl> for Url<'de> {
    fn into(self) -> StdUrl {
        StdUrl::parse(self.0).unwrap()
    }
}

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub enum ImageSize {
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "large")]
    Large,
    #[serde(rename = "extralarge")]
    ExtraLarge,
    #[serde(rename = "mega")]
    Mega,
    #[serde(rename = "")]
    Default,
}

#[derive(Deserialize, Debug)]
pub struct Image<'dt> {
    #[serde(rename = "#text")]
    pub text: &'dt str,
    pub size: ImageSize,
}

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub enum SearchQueryRole {
    #[serde(rename = "request")]
    Request,
}

#[derive(Deserialize, Debug)]
pub struct SearchQuery<'dt> {
    pub role: SearchQueryRole,
    #[serde(rename = "searchTerms")]
    #[serde(borrow)]
    pub search_terms: Option<&'dt str>,
    #[serde(rename = "startPage")]
    #[serde(deserialize_with = "str_to_option")]
    pub start_page: Option<u32>,
}

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Id1<'dt> {
    #[serde(rename = "#text")]
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
}

#[derive(Deserialize, Debug)]
pub struct Id2<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Url<'dt>,
    #[serde(default)]
    pub image: Vec<Image<'dt>>,
}

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Streamable {
    #[serde(rename = "#text")]
    #[serde(deserialize_with = "str_to_val")]
    pub streamable: u32,
    #[serde(deserialize_with = "str_to_val")]
    pub fulltrack: u32,
}

#[derive(Deserialize, Debug)]
pub struct Rank {
    #[serde(deserialize_with = "str_to_val")]
    pub rank: u32,
}


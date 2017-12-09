use std::convert::Into;
use std::str::FromStr;
use std::result::Result as StdResult;
use std::fmt::Display;

use url::Url as StdUrl;
use serde::de::{Deserialize, Deserializer, Error as SerdeError};
use serde_json as json;

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

use std::error::Error;
use std::convert::{Into, From};
use std::str::FromStr;
use std::result::Result as StdResult;
use std::fmt::{Display, Formatter, Result as FmtResult};

use url::Url as StdUrl;
use serde::de::{Deserialize, Deserializer, Visitor, Error as SerdeError};
use serde_json as json;

// ----------------------------------------------------------------

#[derive(Debug)]
pub struct Url {
    pub url: StdUrl,
}

impl<'de> Deserialize<'de> for Url {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UrlVisitor;
        impl<'de> Visitor<'de> for UrlVisitor {
            type Value = Url;

            fn expecting(&self, f: &mut Formatter) -> FmtResult {
                write!(f, "url string")
            }

            fn visit_str<E>(self, c: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let url = StdUrl::parse(c).unwrap(); // do something about it
                Ok(Url::from(url))
            }
        }
        deserializer.deserialize_str(UrlVisitor)
    }
}

impl Into<StdUrl> for Url {
    fn into(self) -> StdUrl {
        self.url
    }
}

impl From<StdUrl> for Url {
    fn from(value: StdUrl) -> Url {
        Url { url: value }
    }
}

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
pub struct SearchQuery {
    pub role: SearchQueryRole,
    #[serde(rename = "searchTerms")]
    pub search_terms: Option<String>,
    #[serde(rename = "startPage")]
    pub start_page: u32,
}

// Generates a wrapper over opensearch object
// Following should be included in order to use this macro:
// use common::SearchQuery;
#[macro_export]
macro_rules! search_t {
    ($name:ident, $matches:ident, $matches_t:ty) => {
        #[derive(Deserialize, Debug)]
        pub struct $name {
            #[serde(rename="opensearch:Query")]
            pub query: SearchQuery,
            #[serde(rename="opensearch:totalResults")]
            pub total_results: u32,
            #[serde(rename="opensearch:startIndex")]
            pub start_index: u32,
            #[serde(rename="opensearch:itemsPerPage")]
            pub iterms_per_page: u32,
            pub $matches: Option<$matches_t>,
        }
    };
}

use std::error::Error;
use std::convert::{Into, From};
use std::marker::Sized;
use std::fmt::{Formatter, Result as FmtResult};
use url::Url as StdUrl;
use serde::de::{Deserialize, Deserializer, Visitor};

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Token {
    pub token: String,
}

pub trait Wrapped<'de>
    where Self: Sized
{
    type Outer: Deserialize<'de> + Into<Self>;
}

// Generates a wrapper over root json object
// Following should be included in order to use this macro:
// use std::convert::Into;
// use common::Wrapped;
#[macro_export]
macro_rules! wrapper_t {
    ($name:ident, $wrapped:ident, $wrapped_t:ty) => {
        #[derive(Deserialize, Debug)]
        pub struct $name {
            $wrapped: $wrapped_t,
        }
        impl Into<$wrapped_t> for $name {
            fn into(self) -> $wrapped_t {
                self.$wrapped
            }
        }
        impl<'de> Wrapped<'de> for $wrapped_t {
            type Outer = $name;
        }
    }
}

// ----------------------------------------------------------------

#[derive(Debug)]
pub struct Url {
    pub url: StdUrl,
}

impl<'de> Deserialize<'de> for Url {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct UrlVisitor;
        impl<'de> Visitor<'de> for UrlVisitor {
            type Value = Url;

            fn expecting(&self, f: &mut Formatter) -> FmtResult {
                write!(f, "url string")
            }

            fn visit_str<E>(self, c: &str) -> Result<Self::Value, E>
                where E: Error
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

#[derive(Deserialize, Debug)]
pub enum ImageSize {
    #[serde(rename="small")]
    Small,
    #[serde(rename="medium")]
    Medium,
    #[serde(rename="large")]
    Large,
    #[serde(rename="extralarge")]
    ExtraLarge,
    #[serde(rename="mega")]
    Mega,
    #[serde(rename="")]
    Default,
}

#[derive(Deserialize, Debug)]
pub struct Image {
    #[serde(rename="#text")]
    pub text: String,
    pub size: ImageSize,
}

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub enum SearchQueryRole {
    #[serde(rename="request")]
    Request,
}

#[derive(Deserialize, Debug)]
pub struct SearchQuery {
    pub role: SearchQueryRole,
    #[serde(rename="searchTerms")]
    pub search_terms: Option<String>,
    #[serde(rename="startPage")]
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
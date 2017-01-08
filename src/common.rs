use std::error::Error;
use std::convert::{Into, From};
use url::Url as StdUrl;
use serde::de::{Deserialize, Deserializer, Visitor};

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct PoundText {
    #[serde(rename="#text")]
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct Token {
    pub token: String,
}

// ----------------------------------------------------------------

#[derive(Debug)]
pub struct Url {
    url: StdUrl,
}

impl Deserialize for Url {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        struct UrlVisitor;
        impl Visitor for UrlVisitor {
            type Value = Url;
            fn visit_str<E>(&mut self, c: &str) -> Result<Self::Value, E>
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

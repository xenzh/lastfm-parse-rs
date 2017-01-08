use super::common::{Url, Image};
use super::tag::Refs as TagRefs;

#[derive(Deserialize, Debug)]
pub struct Stats {
    pub listeners: u32,
    pub playcount: u32,
}

#[derive(Deserialize, Debug)]
pub struct SimilarArtist {
    pub name: String,
    pub url: Url,
    pub image: Vec<Image>,
}

#[derive(Deserialize, Debug)]
pub struct Similar {
    pub artist: Vec<SimilarArtist>,
}

#[derive(Deserialize, Debug)]
pub struct Link {
    #[serde(rename="#text")]
    pub text: String,
    pub rel: String,
    pub href: Url,
}

#[derive(Deserialize, Debug)]
pub struct Links {
    pub link: Link,
}

#[derive(Deserialize, Debug)]
pub struct Bio {
    links: Links,
    published: String, // TODO: change type to actual date (like Url)
    summary: String,
    content: String,
}

/// api methods: artist.getinfo
#[derive(Deserialize, Debug)]
pub struct Info {
    pub name: String,
    pub mbid: String,
    pub url: Url,
    pub image: Vec<Image>,
    pub streamable: u32,
    pub ontour: u32,
    pub stats: Stats,
    pub similar: Similar,
    pub tags: TagRefs,
    pub bio: Bio,
}

#[derive(Deserialize, Debug)]
pub struct Ref {
    pub name: String,
    pub mbid: String,
    pub url: Url,
}
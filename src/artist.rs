use super::common::{Url, Image};
use super::tag::Refs as TagRefs;

use std::convert::Into;
use super::common::Wrapped;

/// api methods: artist.getinfo
wrapper_t!(ArtistInfo, artist, Info);

/// api methods: artist.getsimilar
wrapper_t!(SimilarArtistList, similarartists, SimilarList);

#[derive(Deserialize, Debug)]
pub struct Stats {
    pub listeners: u32,
    pub playcount: u32,
}

#[derive(Deserialize, Debug)]
pub struct SimilarInfo {
    pub name: String,
    pub url: Url,
    pub image: Option<Vec<Image>>,
}

#[derive(Deserialize, Debug)]
pub struct SimilarInfoList {
    pub artist: Option<Vec<SimilarInfo>>,
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

#[derive(Deserialize, Debug)]
pub struct Info {
    pub name: String,
    pub mbid: Option<String>,
    pub url: Url,
    pub image: Option<Vec<Image>>,
    pub streamable: u32,
    pub ontour: u32,
    pub stats: Stats,
    pub similar: SimilarInfoList,
    pub tags: TagRefs,
    pub bio: Bio,
}

#[derive(Deserialize, Debug)]
pub struct Ref {
    pub name: String,
    pub mbid: Option<String>,
    pub url: Url,
}

#[derive(Deserialize, Debug)]
pub struct Similar {
    pub name: String,
    pub mbid: Option<String>,
    #[serde(rename="match")]
    pub similar_match: f32,
    pub url: Url,
    pub image: Option<Vec<Image>>,
    pub streamable: u32,
}

#[derive(Deserialize, Debug)]
pub struct SimilarList {
    pub artist: Option<Vec<Similar>>,
}
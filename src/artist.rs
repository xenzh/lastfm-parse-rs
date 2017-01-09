use super::common::{Url, Image};
use super::tag::Refs as TagRefs;

use std::convert::Into;
use super::common::Wrapped;

/// api methods: artist.getinfo
wrapper_t!(ArtistInfo, artist, Info);

/// api methods: artist.getsimilar
wrapper_t!(SimilarArtistList, similarartists, SimilarArtists);

#[derive(Deserialize, Debug)]
pub struct Stats {
    pub listeners: u32,
    pub playcount: u32,
}

#[derive(Deserialize, Debug)]
pub struct SimilarArtistInfo {
    pub name: String,
    pub url: Url,
    pub image: Vec<Image>,
}

#[derive(Deserialize, Debug)]
pub struct SimilarInfo {
    pub artist: Vec<SimilarArtistInfo>,
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
    pub image: Vec<Image>,
    pub streamable: u32,
    pub ontour: u32,
    pub stats: Stats,
    pub similar: SimilarInfo,
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
pub struct SimilarArtist {
    pub name: String,
    pub mbid: Option<String>,
    #[serde(rename="match")]
    pub similar_match: f32,
    pub url: Url,
    pub image: Vec<Image>,
    pub streamable: u32,
}

#[derive(Deserialize, Debug)]
pub struct SimilarArtists {
    pub artist: Vec<SimilarArtist>,
}
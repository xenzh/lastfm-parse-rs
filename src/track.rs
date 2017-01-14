use super::common::{Url, Image};
use super::artist::Ref as ArtistRef;
use super::album::Ref as AlbumRef;
use super::tag::Refs as TagRefs;

use std::convert::Into;
use super::common::Wrapped;

/// api methods: track.getinfo
wrapper_t!(TrackInfo, track, Info);

/// api methods: track.getsimilar
wrapper_t!(TrackSimilarList, similartracks, SimilarList);

#[derive(Deserialize, Debug)]
pub struct Info {
    pub name: String,
    pub mbid: Option<String>,
    pub url: Url,
    pub duration: Option<u32>,
    // streamable is omitted for now
    pub listeners: u32,
    pub playcount: u32,
    pub artist: ArtistRef,
    pub album: AlbumRef,
    pub toptags: TagRefs,
}

#[derive(Deserialize, Debug)]
pub struct Ref {
    pub name: String,
    pub url: Url,
    pub duration: u32, // artist, streamable and @attr fields omitted for now
}

#[derive(Deserialize, Debug)]
pub struct Refs {
    pub track: Option<Vec<Ref>>,
}

#[derive(Deserialize, Debug)]
pub struct Similar {
    name: String,
    playcount: u32,
    mbid: Option<String>,
    #[serde(rename="match")]
    trackmatch: f32,
    url: Url,
    // streamable is omitted for now
    duration: Option<u32>,
    artist: ArtistRef,
    image: Option<Vec<Image>>,
}

#[derive(Deserialize, Debug)]
pub struct SimilarList {
    track: Option<Vec<Similar>>,
}
use super::common::{Url, Image};
use super::track::Refs as TrackRefs;
use super::tag::Refs as TagRefs;

use std::convert::Into;
use super::common::Wrapped;

/// api methods: album.getinfo
wrapper_t!(AlbumInfo, album, Info);

#[derive(Deserialize, Debug)]
pub struct Info {
    pub name: String,
    pub artist: String,
    pub mbid: Option<String>,
    pub url: Url,
    pub image: Option<Vec<Image>>,
    pub listeners: u32,
    pub playcount: u32,
    pub tracks: TrackRefs,
    pub tags: TagRefs,
}

#[derive(Deserialize, Debug)]
pub struct Ref {
    pub artist: String,
    pub title: String,
    pub mbid: String,
    pub url: Url,
    pub image: Option<Vec<Image>>, // attr position is omitted for now
}
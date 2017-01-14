use std::convert::Into;
use super::common::{Url, Image, Wrapped, SearchQuery};
use super::track::Refs as TrackRefs;
use super::tag::Refs as TagRefs;


/// api methods: album.getinfo
wrapper_t!(AlbumInfo, album, Info);

/// api methods: album.search
search_t!(Search, albummatches, SearchRefs);
wrapper_t!(AlbumSearch, results, Search);


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
    pub mbid: Option<String>,
    pub url: Url,
    pub image: Option<Vec<Image>>, // attr position is omitted for now
}

#[derive(Deserialize, Debug)]
pub struct SearchRef {
    pub name: String,
    pub artist: String,
    pub url: Url,
    pub image: Option<Vec<Image>>,
    pub streamable: u32,
    pub mbid: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SearchRefs {
    pub album: Option<Vec<SearchRef>>,
}
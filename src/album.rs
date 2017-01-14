use std::convert::Into;
use super::common::{Url, Image, Wrapped, SearchQuery};
use super::artist::Ref as ArtistRef;
use super::track::Refs as TrackRefs;
use super::tag::Refs as TagRefs;


/// api methods: album.getinfo
wrapper_t!(AlbumInfo, album, Info);

/// api methods: album.search
search_t!(Search, albummatches, SearchRefs);
wrapper_t!(AlbumSearch, results, Search);

/// api methods: artist.gettopalbums
wrapper_t!(TopAlbums, topalbums, TopRefs);

/// api methods: tag.gettopalbums
wrapper_t!(Albums, albums, Refs);


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

#[derive(Deserialize, Debug)]
pub struct TopRef {
    name: String,
    playcount: Option<u32>,
    mbid: Option<String>,
    url: Url,
    artist: ArtistRef,
    image: Option<Vec<Image>>,
}

#[derive(Deserialize, Debug)]
pub struct TopRefs {
    album: Option<Vec<TopRef>>,
}

#[derive(Deserialize, Debug)]
pub struct Refs {
    album: Option<Vec<TopRef>>,
}
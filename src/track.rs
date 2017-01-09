use super::common::Url;
use super::artist::Ref as ArtistRef;
use super::album::Ref as AlbumRef;
use super::tag::Refs as TagRefs;

/// api methods: track.getinfo
wrapper_t!(track, Info);

#[derive(Deserialize, Debug)]
pub struct Info {
    pub name: String,
    pub mbid: String,
    pub url: Url,
    pub duration: u32,
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
    pub track: Vec<Ref>,
}
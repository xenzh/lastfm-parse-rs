use std::convert::Into;
use super::common::{Url, Image, Wrapped, SearchQuery};
use super::artist::Ref as ArtistRef;
use super::album::Ref as AlbumRef;
use super::tag::Refs as TagRefs;


/// api methods: track.getinfo
wrapper_t!(TrackInfo, track, Info);

/// api methods: track.getsimilar
wrapper_t!(TrackSimilarList, similartracks, SimilarList);

/// api methods: track.search
search_t!(Search, trackmatches, SearchRefs);
wrapper_t!(TrackSearch, results, Search);

/// api methods: artist.gettoptracks
wrapper_t!(TopTracks, toptracks, TopRefs);

/// api methods: chart.gettoptracks
wrapper_t!(ChartTopTracks, tracks, ChartRefs);

/// api methods: geo.gettoptracks
wrapper_t!(GeoTopTracks, tracks, GeoRefs);

/// api methods: tag.gettoptracks
wrapper_t!(TagTopTracks, tracks, TagTopRefs);

/// api methods: track.getcorrection
wrapper_t!(TrackCorrections, corrections, Corrections);


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
    pub album: Option<AlbumRef>,
    pub toptags: Option<TagRefs>,
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
    pub name: String,
    pub playcount: u32,
    pub mbid: Option<String>,
    #[serde(rename="match")]
    pub trackmatch: f32,
    pub url: Url,
    // streamable is omitted for now
    pub duration: Option<u32>,
    pub artist: ArtistRef,
    pub image: Option<Vec<Image>>,
}

#[derive(Deserialize, Debug)]
pub struct SimilarList {
    pub track: Option<Vec<Similar>>,
}

#[derive(Deserialize, Debug)]
pub struct SearchRef {
    pub name: String,
    pub artist: String,
    pub url: Url,
    // pub streamable: u32, // seems to have server problems now
    pub listeners: u32,
    pub image: Option<Vec<Image>>,
}

#[derive(Deserialize, Debug)]
pub struct SearchRefs {
    pub track: Option<Vec<SearchRef>>,
}

#[derive(Deserialize, Debug)]
pub struct TopRef {
    pub name: String,
    pub playcount: u32,
    pub listeners: u32,
    pub mbid: Option<String>,
    pub url: Url,
    pub streamable: u32,
    pub artist: ArtistRef,
    pub image: Option<Vec<Image>>,
}

#[derive(Deserialize, Debug)]
pub struct TopRefs {
    pub track: Option<Vec<TopRef>>,
}

#[derive(Deserialize, Debug)]
pub struct ChartRefs {
    pub track: Option<Vec<Info>>,
}

#[derive(Deserialize, Debug)]
pub struct GeoRef {
    pub name: String,
    pub duration: Option<u32>,
    pub listeners: u32,
    pub mbid: Option<String>,
    pub url: Url,
    pub artist: ArtistRef,
    pub image: Option<Vec<Image>>,
}

#[derive(Deserialize, Debug)]
pub struct GeoRefs {
    pub track: Option<Vec<GeoRef>>,
}

#[derive(Deserialize, Debug)]
pub struct TagTopRef {
    pub name: String,
    pub duration: Option<u32>,
    pub mbid: Option<String>,
    pub url: Url,
    pub artist: ArtistRef,
    pub image: Option<Vec<Image>>,
}

#[derive(Deserialize, Debug)]
pub struct TagTopRefs {
    pub track: Option<Vec<TagTopRef>>,
}

#[derive(Deserialize, Debug)]
pub struct CorrectionTrack {
    pub name: Option<String>,
    pub url: Url,
    pub artist: ArtistRef,
}

#[derive(Deserialize, Debug)]
pub struct Correction {
    pub track: CorrectionTrack,
}

#[derive(Deserialize, Debug)]
pub struct Corrections {
    correction: Correction,
}
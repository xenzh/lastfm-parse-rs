use std::convert::Into;

use url::Url as StdUrl;

use lastfm_type::{LastfmType, Request, RequestParams};
use super::common::{Url, Image, Id2, Streamable, str_to_option, str_to_val};

// ----------------------------------------------------------------

#[derive(Debug)]
pub enum Params<'dt> {
    Phantom(&'dt ()),
    GetTopArtists {
        limit: Option<u32>,
        page: Option<u32>,
    },
    GetTopTags {
        limit: Option<u32>,
        page: Option<u32>,
    },
    GetTopTracks {
        limit: Option<u32>,
        page: Option<u32>,
    },
}

impl<'pr> RequestParams for Params<'pr> {
    fn method(&self) -> &str {
        match *self {
            Params::GetTopArtists { .. } => "chart.gettopartists",
            Params::GetTopTags { .. } => "chart.gettoptags",
            Params::GetTopTracks { .. } => "chart.gettoptracks",
            Params::Phantom(_) => panic!("this is a dummy item"),
        }
    }

    fn is_write(&self) -> bool {
        false
    }

    fn append_to(&self, url: &mut StdUrl) {
        let mut query = url.query_pairs_mut();
        match *self {
            Params::GetTopArtists { limit, page } => {
                if let Some(limit) = limit {
                    query.append_pair("limit", &limit.to_string());
                }
                if let Some(page) = page {
                    query.append_pair("page", &page.to_string());
                }
            }
            Params::GetTopTags { limit, page } => {
                if let Some(limit) = limit {
                    query.append_pair("limit", &limit.to_string());
                }
                if let Some(page) = page {
                    query.append_pair("page", &page.to_string());
                }
            }
            Params::GetTopTracks { limit, page } => {
                if let Some(limit) = limit {
                    query.append_pair("limit", &limit.to_string());
                }
                if let Some(page) = page {
                    query.append_pair("page", &page.to_string());
                }
            }
            Params::Phantom(_) => panic!("this is a dummy item"),
        }
    }
}

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Artist<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Url<'dt>,
    #[serde(deserialize_with = "str_to_val")]
    pub playcount: u32,
    #[serde(deserialize_with = "str_to_val")]
    pub listeners: u32,
    #[serde(default)]
    #[serde(deserialize_with = "str_to_option")]
    pub streamable: Option<u32>,
    pub image: Vec<Image<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct GetTopArtists<'dt> {
    #[serde(borrow)]
    pub artist: Option<Vec<Artist<'dt>>>,
}

lastfm_t!(
    artists,
    GetTopArtists,
    _TopArtists,
    Params,
    GetTopArtists,
    [limit: Option<u32>, page: Option<u32>]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Tag<'dt> {
    pub name: &'dt str,
    pub url: Url<'dt>,
    #[serde(deserialize_with = "str_to_val")]
    pub reach: u32,
    #[serde(deserialize_with = "str_to_val")]
    pub taggings: u32,
    #[serde(deserialize_with = "str_to_val")]
    pub streamable: u32, 
    // wiki omitted for now: service always returns {}.
}

#[derive(Deserialize, Debug)]
pub struct GetTopTags<'dt> {
    #[serde(borrow)]
    pub tag: Option<Vec<Tag<'dt>>>,
}

lastfm_t!(
    tags,
    GetTopTags,
    _TopTags,
    Params,
    GetTopTags,
    [limit: Option<u32>, page: Option<u32>]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Track<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Url<'dt>,
    #[serde(deserialize_with = "str_to_val")]
    pub duration: u32,
    #[serde(deserialize_with = "str_to_val")]
    pub playcount: u32,
    #[serde(deserialize_with = "str_to_val")]
    pub listeners: u32,
    pub streamable: Streamable,
    pub artist: Id2<'dt>,
    pub image: Vec<Image<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct GetTopTracks<'dt> {
    #[serde(borrow)]
    pub track: Option<Vec<Track<'dt>>>,
}

lastfm_t!(
    tracks,
    GetTopTracks,
    _TopTracks,
    Params,
    GetTopTracks,
    [limit: Option<u32>, page: Option<u32>]
);

use std::convert::Into;

use url::Url as StdUrl;

use lastfm_type::{LastfmType, Request, RequestParams};
use super::common::{Url, Image, Id2, Streamable, Rank, str_to_val};

// ----------------------------------------------------------------

#[derive(Debug)]
pub enum Params<'pr> {
    Phantom(&'pr ()),
    GetTopArtists {
        country: &'pr str,
        limit: Option<u32>,
        page: Option<u32>,
    },
    GetTopTracks {
        country: &'pr str,
        location: Option<&'pr str>,
        limit: Option<u32>,
        page: Option<u32>,
    },
}

impl<'pr> RequestParams for Params<'pr> {
    fn method(&self) -> &str {
        match *self {
            Params::GetTopArtists { .. } => "geo.gettopartists",
            Params::GetTopTracks { .. } => "geo.gettoptracks",
            Params::Phantom(_) => panic!("this is a dummy item")
        }
    }

    fn needs_signature(&self) -> bool {
        false
    }

    fn needs_session_key(&self) -> bool {
        false
    }

    fn append_to(&self, url: &mut StdUrl) {
        let mut query = url.query_pairs_mut();
        match *self {
            Params::GetTopArtists {
                country,
                limit,
                page,
            } => {
                query.append_pair("country", country);
                if let Some(limit) = limit {
                    query.append_pair("limit", &limit.to_string());
                }
                if let Some(page) = page {
                    query.append_pair("page", &page.to_string());
                }
            }
            Params::GetTopTracks {
                country,
                location,
                limit,
                page,
            } => {
                query.append_pair("country", country);
                if let Some(location) = location {
                    query.append_pair("location", location);
                }
                if let Some(limit) = limit {
                    query.append_pair("limit", &limit.to_string());
                }
                if let Some(page) = page {
                    query.append_pair("page", &page.to_string());
                }
            }
            Params::Phantom(_) => panic!("this is a dummy item")
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
    pub streamable: u32,
    pub image: Vec<Image<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct GetTopArtists<'dt> {
    #[serde(borrow)]
    pub artist: Option<Vec<Artist<'dt>>>,
}

lastfm_t!(
    topartists,
    GetTopArtists,
    _TopArtists,
    Params,
    GetTopArtists,
    [country: &'rq str, limit: Option<u32>, page: Option<u32>]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Track<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Url<'dt>,
    pub streamable: Option<Streamable>,
    pub artist: Id2<'dt>,
    pub image: Vec<Image<'dt>>,
    #[serde(rename = "@attr")]
    pub rank: Option<Rank>,
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
    [
        country: &'rq str,
        location: Option<&'rq str>,
        limit: Option<u32>,
        page: Option<u32>
    ]
);

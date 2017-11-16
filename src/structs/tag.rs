use std::convert::Into;

use url::Url;

use methods::Method;
use lastfm_type::{LastfmType, Request, RequestParams};
//use super::common::Url as LastfmUrl;

// ----------------------------------------------------------------

#[derive(Debug)]
pub enum Params<'pr> {
    GetInfo { tag: &'pr str },
    GetSimilar { tag: &'pr str },
    GetTopAlbums {
        tag: &'pr str,
        limit: Option<u32>,
        page: Option<u32>,
    },
    GetTopArtists {
        tag: &'pr str,
        limit: Option<u32>,
        page: Option<u32>,
    },
    GetTopTags,
    GetTopTracks {
        tag: &'pr str,
        limit: Option<u32>,
        page: Option<u32>,
    },
    GetWeeklyChartList { tag: &'pr str },
}

impl<'pr> RequestParams for Params<'pr> {
    fn append_to(&self, url: &mut Url) {
        let mut query = url.query_pairs_mut();
        match *self {
            Params::GetInfo { tag } => {
                query.append_pair("tag", tag);
            }
            Params::GetSimilar { tag } => {
                query.append_pair("tag", tag);
            }
            Params::GetTopAlbums { tag, limit, page } => {
                query.append_pair("tag", tag);
                if let Some(lim) = limit {
                    query.append_pair("limit", &lim.to_string());
                }
                if let Some(pg) = page {
                    query.append_pair("page", &pg.to_string());
                }
            }
            Params::GetTopArtists { tag, limit, page } => {
                query.append_pair("tag", tag);
                if let Some(lim) = limit {
                    query.append_pair("limit", &lim.to_string());
                }
                if let Some(pg) = page {
                    query.append_pair("page", &pg.to_string());
                }
            }
            Params::GetTopTags => {}
            Params::GetTopTracks { tag, limit, page } => {
                query.append_pair("tag", tag);
                if let Some(lim) = limit {
                    query.append_pair("limit", &lim.to_string());
                }
                if let Some(pg) = page {
                    query.append_pair("page", &pg.to_string());
                }
            }
            Params::GetWeeklyChartList { tag } => {
                query.append_pair("tag", tag);
            }
        };
    }
}

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Wiki<'dt> {
    pub summary: &'dt str,
    pub content: Option<&'dt str>,
}

#[derive(Deserialize, Debug)]
pub struct Info<'dt> {
    pub name: &'dt str,
    pub total: u32,
    pub reach: u32,
    #[serde(borrow)]
    pub wiki: Wiki<'dt>,
}

lastfm_t!(
    tag,
    Info,
    _Info,
    Method,
    TagGetInfo,
    Params,
    GetInfo,
    [tag: &'rq str]
);

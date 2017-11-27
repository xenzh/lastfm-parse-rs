use std::convert::Into;
use std::marker::PhantomData;

use url::Url as StdUrl;

use methods::Method;
use lastfm_type::{LastfmType, Request, RequestParams};
use super::common::{Url, Image, str_to_option};

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
    fn append_to(&self, url: &mut StdUrl) {
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
    pub total: Option<u32>,
    pub reach: Option<u32>,
    #[serde(borrow)]
    pub wiki: Option<Wiki<'dt>>,
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

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Similar<'dt> {
    pub name: &'dt str,
    pub url: Option<Url<'dt>>,
    pub streamable: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct SimilarList<'dt> {
    #[serde(borrow)]
    pub tag: Option<Vec<Similar<'dt>>>,
}

lastfm_t!(
    similartags,
    SimilarList,
    _SimilarList,
    Method,
    TagGetSimilar,
    Params,
    GetSimilar,
    [tag: &'rq str]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Artist1<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Option<Url<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct Album<'dt> {
    name: &'dt str,
    playcount: Option<u32>,
    mbid: Option<String>,
    url: Option<Url<'dt>>,
    #[serde(borrow)]
    artist: Option<Artist1<'dt>>,
    #[serde(borrow)]
    image: Option<Vec<Image<'dt>>>,
}

#[derive(Deserialize, Debug)]
pub struct TopAlbums<'dt> {
    #[serde(borrow)]
    album: Option<Vec<Album<'dt>>>,
}

lastfm_t!(
    albums,
    TopAlbums,
    _TopAlbums,
    Method,
    TagGetTopAlbums,
    Params,
    GetTopAlbums,
    [tag: &'rq str, limit: Option<u32>, page: Option<u32>]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Artist2<'dt> {
    pub name: &'dt str,
    pub url: Option<Url<'dt>>,
    #[serde(deserialize_with = "str_to_option")]
    pub streamable: Option<u32>,
    #[serde(borrow)]
    pub image: Option<Vec<Image<'dt>>>,
}

#[derive(Deserialize, Debug)]
pub struct TopArtists<'dt> {
    #[serde(borrow)]
    pub artist: Option<Vec<Artist2<'dt>>>,
}

lastfm_t!(
    topartists,
    TopArtists,
    _TopArtists,
    Method,
    TagGetTopArtists,
    Params,
    GetTopArtists,
    [tag: &'rq str, limit: Option<u32>, page: Option<u32>]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Tag<'dt> {
    pub name: &'dt str,
    pub count: Option<u32>,
    pub url: Option<Url<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct TopTags<'dt> {
    #[serde(borrow)]
    pub tag: Option<Vec<Tag<'dt>>>,
}

lastfm_t!(
    toptags,
    TopTags,
    _TopTags,
    Method,
    TagGetTopTags,
    Params,
    GetTopTags,
    []
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Track<'dt> {
    pub name: &'dt str,
    #[serde(deserialize_with="str_to_option")]
    pub duration: Option<u32>,
    pub mbid: Option<&'dt str>,
    pub url: Option<Url<'dt>>,
    #[serde(borrow)]
    pub artist: Option<Artist1<'dt>>,
    #[serde(borrow)]
    pub image: Option<Vec<Image<'dt>>>,
}

#[derive(Deserialize, Debug)]
pub struct TopTracks<'dt> {
    #[serde(borrow)]
    pub track: Option<Vec<Track<'dt>>>,
}

lastfm_t!(
    tracks,
    TopTracks,
    _TopTracks,
    Method,
    TagGetTopTracks,
    Params,
    GetTopTracks,
    [tag: &'rq str, limit: Option<u32>, page: Option<u32>]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct WeeklyChartItem {
    #[serde(deserialize_with="str_to_option")]
    pub from: Option<u32>,
    #[serde(deserialize_with="str_to_option")]
    pub to: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct WeeklyChartList<'dt> {
    #[serde(skip)]
    phantom: PhantomData<&'dt ()>,
    chart: Option<Vec<WeeklyChartItem>>,
}

lastfm_t!(
    weeklychartlist,
    WeeklyChartList,
    _WeeklyChartList,
    Method,
    TagGetWeeklyChartList,
    Params,
    GetWeeklyChartList,
    [tag: &'rq str]
);
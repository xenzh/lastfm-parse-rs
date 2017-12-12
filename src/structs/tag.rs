use std::convert::Into;
use std::marker::PhantomData;

use url::Url as StdUrl;

use lastfm_type::{LastfmType, Request, RequestParams};
use super::common::{Url, Image, str_to_option, str_to_val};

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
    fn method(&self) -> &str {
        match *self {
            Params::GetInfo { .. } => "tag.getinfo",
            Params::GetSimilar { .. } => "tag.getsimilar",
            Params::GetTopAlbums { .. } => "tag.gettopalbums",
            Params::GetTopArtists { .. } => "tag.gettopartists",
            Params::GetTopTags { .. } => "tag.gettoptags",
            Params::GetTopTracks { .. } => "tag.gettoptracks",
            Params::GetWeeklyChartList { .. } => "tag.getweeklychartlist",
        }
    }

    fn is_write(&self) -> bool {
        false
    }

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
pub struct GetInfo<'dt> {
    pub name: &'dt str,
    pub total: u32,
    pub reach: u32,
    #[serde(borrow)]
    pub wiki: Option<Wiki<'dt>>,
}

lastfm_t!(
    tag,
    GetInfo,
    _Info,
    Params,
    GetInfo,
    [tag: &'rq str]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Similar<'dt> {
    pub name: &'dt str,
    pub url: Url<'dt>,
    pub streamable: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct GetSimilar<'dt> {
    #[serde(borrow)]
    pub tag: Option<Vec<Similar<'dt>>>,
}

lastfm_t!(
    similartags,
    GetSimilar,
    _SimilarList,
    Params,
    GetSimilar,
    [tag: &'rq str]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Artist1<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Url<'dt>,
}

#[derive(Deserialize, Debug)]
pub struct Album<'dt> {
    name: &'dt str,
    mbid: Option<&'dt str>,
    url: Url<'dt>,
    playcount: Option<u32>,
    artist: Artist1<'dt>,
    image: Vec<Image<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct GetTopAlbums<'dt> {
    #[serde(borrow)]
    album: Option<Vec<Album<'dt>>>,
}

lastfm_t!(
    albums,
    GetTopAlbums,
    _TopAlbums,
    Params,
    GetTopAlbums,
    [tag: &'rq str, limit: Option<u32>, page: Option<u32>]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Artist2<'dt> {
    pub name: &'dt str,
    pub url: Url<'dt>,
    #[serde(default)]
    #[serde(deserialize_with = "str_to_option")]
    pub streamable: Option<u32>,
    pub image: Vec<Image<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct GetTopArtists<'dt> {
    #[serde(borrow)]
    pub artist: Option<Vec<Artist2<'dt>>>,
}

lastfm_t!(
    topartists,
    GetTopArtists,
    _TopArtists,
    Params,
    GetTopArtists,
    [tag: &'rq str, limit: Option<u32>, page: Option<u32>]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Tag<'dt> {
    pub name: &'dt str,
    pub count: u32,
    pub reach: u32,
}

#[derive(Deserialize, Debug)]
pub struct GetTopTags<'dt> {
    #[serde(borrow)]
    pub tag: Option<Vec<Tag<'dt>>>,
}

lastfm_t!(
    toptags,
    GetTopTags,
    _TopTags,
    Params,
    GetTopTags,
    []
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Track<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Url<'dt>,
    #[serde(deserialize_with = "str_to_val")]
    pub duration: u32,
    pub artist: Artist1<'dt>,
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
    [tag: &'rq str, limit: Option<u32>, page: Option<u32>]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct WeeklyChartItem {
    #[serde(deserialize_with = "str_to_val")]
    pub from: u32,
    #[serde(deserialize_with = "str_to_val")]
    pub to: u32,
}

#[derive(Deserialize, Debug)]
pub struct GetWeeklyChartList<'dt> {
    #[serde(skip)]
    phantom: PhantomData<&'dt ()>,
    chart: Option<Vec<WeeklyChartItem>>,
}

lastfm_t!(
    weeklychartlist,
    GetWeeklyChartList,
    _WeeklyChartList,
    Params,
    GetWeeklyChartList,
    [tag: &'rq str]
);

use std::convert::Into;

use url::Url as StdUrl;

use lastfm_type::{LastfmType, Request, RequestParams};
use super::common::{Url, Image, SearchQuery, str_to_option, str_to_val};

// ----------------------------------------------------------------

#[derive(Debug)]
pub enum Params<'pr> {
    AddTags { artist: &'pr str, tags: &'pr str }, // auth
    GetCorrection { artist: &'pr str },
    GetInfo {
        artist: &'pr str,
        mbid: Option<&'pr str>,
        autocorrect: Option<u32>,
        lang: Option<&'pr str>,
        username: Option<&'pr str>,
    },
    GetSimilar {
        artist: &'pr str,
        mbid: Option<&'pr str>,
        autocorrect: Option<u32>,
        limit: Option<u32>,
    },
    GetTags {
        artist: &'pr str,
        mbid: Option<&'pr str>,
        autocorrect: Option<u32>,
        user: Option<&'pr str>,
    },
    GetTopAlbums {
        artist: &'pr str,
        mbid: Option<&'pr str>,
        autocorrect: Option<u32>,
        limit: Option<u32>,
        page: Option<u32>,
    },
    GetTopTags {
        artist: &'pr str,
        mbid: Option<&'pr str>,
        autocorrect: Option<u32>,
    },
    GetTopTracks {
        artist: &'pr str,
        mbid: Option<&'pr str>,
        autocorrect: Option<u32>,
        limit: Option<u32>,
        page: Option<u32>,
    },
    RemoveTag { artist: &'pr str, tag: &'pr str },
    Search {
        artist: &'pr str,
        limit: Option<u32>,
        page: Option<u32>,
    },
}

impl<'pr> RequestParams for Params<'pr> {
    fn method(&self) -> &str {
        match *self {
            Params::AddTags { .. } => "artist.addtags",
            Params::GetCorrection { .. } => "artist.getcorrection",
            Params::GetInfo { .. } => "artist.getinfo",
            Params::GetSimilar { .. } => "artist.getsimilar",
            Params::GetTags { .. } => "artist.gettags",
            Params::GetTopAlbums { .. } => "artist.gettopalbums",
            Params::GetTopTags { .. } => "artist.gettoptags",
            Params::GetTopTracks { .. } => "artist.gettoptracks",
            Params::RemoveTag { .. } => "artist.removetag",
            Params::Search { .. } => "artist.search",
        }
    }

    fn needs_signature(&self) -> bool {
        match *self {
            Params::AddTags { .. } => true,
            Params::RemoveTag { .. } => true,
            _ => false,
        }
    }

    fn needs_session_key(&self) -> bool {
        self.needs_signature()
    }

    fn append_to(&self, url: &mut StdUrl) {
        let mut query = url.query_pairs_mut();
        match *self {
            Params::AddTags { artist, tags } => {
                query.append_pair("artist", artist);
                query.append_pair("tags", tags);
            }
            Params::GetCorrection { artist } => {
                query.append_pair("artist", artist);
            }
            Params::GetInfo {
                artist,
                mbid,
                autocorrect,
                lang,
                username,
            } => {
                query.append_pair("artist", artist);
                if let Some(mbid) = mbid {
                    query.append_pair("mbid", mbid);
                }
                if let Some(autocorrect) = autocorrect {
                    query.append_pair("autocorrect", &autocorrect.to_string());
                }
                if let Some(lang) = lang {
                    query.append_pair("lang", lang);
                }
                if let Some(username) = username {
                    query.append_pair("username", username);
                }
            }
            Params::GetSimilar {
                artist,
                mbid,
                autocorrect,
                limit,
            } => {
                query.append_pair("artist", artist);
                if let Some(mbid) = mbid {
                    query.append_pair("mbid", mbid);
                }
                if let Some(autocorrect) = autocorrect {
                    query.append_pair("autocorrect", &autocorrect.to_string());
                }
                if let Some(limit) = limit {
                    query.append_pair("limit", &limit.to_string());
                }
            }
            Params::GetTags {
                artist,
                mbid,
                autocorrect,
                user,
            } => {
                query.append_pair("artist", artist);
                if let Some(mbid) = mbid {
                    query.append_pair("mbid", mbid);
                }
                if let Some(autocorrect) = autocorrect {
                    query.append_pair("autocorrect", &autocorrect.to_string());
                }
                if let Some(user) = user {
                    query.append_pair("user", user);
                }
            }
            Params::GetTopAlbums {
                artist,
                mbid,
                autocorrect,
                limit,
                page,
            } => {
                query.append_pair("artist", artist);
                if let Some(mbid) = mbid {
                    query.append_pair("mbid", mbid);
                }
                if let Some(autocorrect) = autocorrect {
                    query.append_pair("autocorrect", &autocorrect.to_string());
                }
                if let Some(limit) = limit {
                    query.append_pair("limit", &limit.to_string());
                }
                if let Some(page) = page {
                    query.append_pair("page", &page.to_string());
                }
            }
            Params::GetTopTags {
                artist,
                mbid,
                autocorrect,
            } => {
                query.append_pair("artist", artist);
                if let Some(mbid) = mbid {
                    query.append_pair("mbid", mbid);
                }
                if let Some(autocorrect) = autocorrect {
                    query.append_pair("autocorrect", &autocorrect.to_string());
                }
            }
            Params::GetTopTracks {
                artist,
                mbid,
                autocorrect,
                limit,
                page,
            } => {
                query.append_pair("artist", artist);
                if let Some(mbid) = mbid {
                    query.append_pair("mbid", mbid);
                }
                if let Some(autocorrect) = autocorrect {
                    query.append_pair("autocorrect", &autocorrect.to_string());
                }
                if let Some(limit) = limit {
                    query.append_pair("limit", &limit.to_string());
                }
                if let Some(page) = page {
                    query.append_pair("page", &page.to_string());
                }
            }
            Params::RemoveTag { artist, tag } => {
                query.append_pair("artist", artist);
                query.append_pair("tag", tag);
            }
            Params::Search {
                artist,
                limit,
                page,
            } => {
                query.append_pair("artist", artist);
                if let Some(limit) = limit {
                    query.append_pair("limit", &limit.to_string());
                }
                if let Some(page) = page {
                    query.append_pair("page", &page.to_string());
                }
            }
        }
    }
}

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Artist1<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Url<'dt>,
}

#[derive(Deserialize, Debug)]
pub struct Correction<'dt> {
    #[serde(borrow)]
    pub artist: Option<Artist1<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct GetCorrections<'dt> {
    #[serde(borrow)]
    pub correction: Option<Correction<'dt>>,
}

lastfm_t!(
    corrections,
    GetCorrections,
    _Corrections,
    Params,
    GetCorrection,
    [artist: &'rq str]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Stats {
    #[serde(deserialize_with = "str_to_val")]
    pub listeners: u32,
    #[serde(deserialize_with = "str_to_val")]
    pub playcount: u32,
    #[serde(default)]
    #[serde(deserialize_with = "str_to_option")]
    pub userplaycount: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct Similar1<'dt> {
    pub name: &'dt str,
    pub url: Url<'dt>,
    pub image: Vec<Image<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct SimilarList1<'dt> {
    #[serde(borrow)]
    pub artist: Option<Vec<Similar1<'dt>>>,
}

#[derive(Deserialize, Debug)]
pub struct Tag1<'dt> {
    pub name: &'dt str,
    pub url: Url<'dt>,
}

#[derive(Deserialize, Debug)]
pub struct Tags<'dt> {
    #[serde(borrow)]
    pub tag: Option<Vec<Tag1<'dt>>>,
}

#[derive(Deserialize, Debug)]
pub struct Link<'dt> {
    #[serde(rename = "#text")]
    pub text: &'dt str,
    pub rel: &'dt str,
    pub href: Url<'dt>,
}

#[derive(Deserialize, Debug)]
pub struct Links<'dt> {
    #[serde(borrow)]
    pub link: Option<Link<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct Bio<'dt> {
    #[serde(borrow)]
    pub links: Links<'dt>,
    pub published: Option<&'dt str>,
    pub summary: Option<String>, // may have escape sequences, therefore String
    pub content: Option<String>, // may have escape sequences, therefore String
}

#[derive(Deserialize, Debug)]
pub struct GetInfo<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Url<'dt>,
    pub image: Vec<Image<'dt>>,
    #[serde(default)]
    #[serde(deserialize_with = "str_to_option")]
    pub streamable: Option<u32>,
    #[serde(deserialize_with = "str_to_val")]
    pub ontour: u32,
    pub stats: Stats,
    #[serde(borrow)]
    pub similar: SimilarList1<'dt>,
    #[serde(borrow)]
    pub tags: Tags<'dt>,
    pub bio: Bio<'dt>,
}

lastfm_t!(
    artist,
    GetInfo,
    _Info,
    Params,
    GetInfo,
    [
        artist: &'rq str,
        mbid: Option<&'rq str>,
        autocorrect: Option<u32>,
        lang: Option<&'rq str>,
        username: Option<&'rq str>
    ]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Similar2<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    #[serde(rename = "match")]
    #[serde(deserialize_with = "str_to_val")]
    pub similar_match: f32,
    pub url: Url<'dt>,
    pub image: Vec<Image<'dt>>,
    #[serde(default)]
    #[serde(deserialize_with = "str_to_option")]
    pub streamable: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct GetSimilar<'dt> {
    #[serde(borrow)]
    pub artist: Option<Vec<Similar2<'dt>>>,
}

lastfm_t!(
    similarartists,
    GetSimilar,
    _SimilarList,
    Params,
    GetSimilar,
    [
        artist: &'rq str,
        mbid: Option<&'rq str>,
        autocorrect: Option<u32>,
        limit: Option<u32>
    ]
);

// ----------------------------------------------------------------

pub type GetTags<'dt> = Tags<'dt>;

lastfm_t!(
    tags,
    GetTags,
    _UserTags,
    Params,
    GetTags,
    [
        artist: &'rq str,
        mbid: Option<&'rq str>,
        autocorrect: Option<u32>,
        user: Option<&'rq str>
    ]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Album<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Url<'dt>,
    pub playcount: u32,
    pub artist: Artist1<'dt>,
    pub image: Vec<Image<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct GetTopAlbums<'dt> {
    #[serde(borrow)]
    pub album: Option<Vec<Album<'dt>>>,
}

lastfm_t!(
    topalbums,
    GetTopAlbums,
    _TopAlbums,
    Params,
    GetTopAlbums,
    [
        artist: &'rq str,
        mbid: Option<&'rq str>,
        autocorrect: Option<u32>,
        limit: Option<u32>,
        page: Option<u32>
    ]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Tag2<'dt> {
    pub name: &'dt str,
    pub count: u32,
    pub url: Url<'dt>,
}

#[derive(Deserialize, Debug)]
pub struct GetTopTags<'dt> {
    #[serde(borrow)]
    pub tag: Option<Vec<Tag2<'dt>>>,
}

lastfm_t!(
    toptags,
    GetTopTags,
    _TopTags,
    Params,
    GetTopTags,
    [
        artist: &'rq str,
        mbid: Option<&'rq str>,
        autocorrect: Option<u32>
    ]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Artist<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Url<'dt>,
}

#[derive(Deserialize, Debug)]
pub struct Track<'dt> {
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
    pub artist: Artist<'dt>,
    pub image: Vec<Image<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct GetTopTracks<'dt> {
    #[serde(borrow)]
    pub track: Option<Vec<Track<'dt>>>,
}

lastfm_t!(
    toptracks,
    GetTopTracks,
    _TopTracks,
    Params,
    GetTopTracks,
    [
        artist: &'rq str,
        mbid: Option<&'rq str>,
        autocorrect: Option<u32>,
        limit: Option<u32>,
        page: Option<u32>
    ]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Artist2<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Url<'dt>,
    #[serde(deserialize_with = "str_to_val")]
    pub listeners: u32,
    #[serde(default)]
    #[serde(deserialize_with = "str_to_option")]
    pub streamable: Option<u32>,
    pub image: Vec<Image<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct SearchData<'dt> {
    #[serde(borrow)]
    pub artist: Option<Vec<Artist2<'dt>>>,
}

opensearch_t!(
    results,
    Search,
    _Search,
    artistmatches,
    SearchData,
    Params,
    Search,
    [artist: &'rq str, limit: Option<u32>, page: Option<u32>]
);

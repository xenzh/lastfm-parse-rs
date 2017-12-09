use std::convert::Into;

use url::Url as StdUrl;

use methods::Method;
use lastfm_type::{LastfmType, Request, RequestParams};
use super::common::{Url, Image, SearchQuery, str_to_option, str_to_val};

// ----------------------------------------------------------------

#[derive(Debug)]
pub enum Params<'pr> {
    AddTags {
        // auth
        artist: &'pr str,
        album: &'pr str,
        tags: &'pr str,
    },
    GetInfo {
        artist: &'pr str,
        album: &'pr str,
        mbid: Option<&'pr str>,
        autocorrect: Option<u32>,
        lang: Option<&'pr str>,
        username: Option<&'pr str>,
    },
    GetTags {
        artist: &'pr str,
        album: &'pr str,
        mbid: Option<&'pr str>,
        autocorrect: Option<u32>,
        user: Option<&'pr str>,
    },
    GetTopTags {
        artist: &'pr str,
        album: &'pr str,
        mbid: Option<&'pr str>,
        autocorrect: Option<u32>,
    },
    RemoveTag {
        artist: &'pr str,
        album: &'pr str,
        tag: &'pr str,
    },
    Search {
        album: &'pr str,
        limit: Option<u32>,
        page: Option<u32>,
    },
}

impl<'pr> RequestParams for Params<'pr> {
    fn append_to(&self, url: &mut StdUrl) {
        let mut query = url.query_pairs_mut();
        match *self {
            Params::AddTags {
                artist,
                album,
                tags,
            } => {
                query.append_pair("artist", artist);
                query.append_pair("album", album);
                query.append_pair("tags", tags);
            }
            Params::GetInfo {
                artist,
                album,
                mbid,
                autocorrect,
                lang,
                username,
            } => {
                query.append_pair("artist", artist);
                query.append_pair("album", album);
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
            Params::GetTags {
                artist,
                album,
                mbid,
                autocorrect,
                user,
            } => {
                query.append_pair("artist", artist);
                query.append_pair("album", album);
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
            Params::GetTopTags {
                artist,
                album,
                mbid,
                autocorrect,
            } => {
                query.append_pair("artist", artist);
                query.append_pair("album", album);
                if let Some(mbid) = mbid {
                    query.append_pair("mbid", mbid);
                }
                if let Some(autocorrect) = autocorrect {
                    query.append_pair("autocorrect", &autocorrect.to_string());
                }
            }
            Params::RemoveTag { artist, album, tag } => {
                query.append_pair("artist", artist);
                query.append_pair("album", album);
                query.append_pair("tag", tag);
            }
            Params::Search { album, limit, page } => {
                query.append_pair("album", album);
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
pub struct Track<'dt> {
    pub name: &'dt str,
    pub url: Url<'dt>,
    #[serde(deserialize_with = "str_to_val")]
    pub duration: u32, 
    // artist, streamable and @attr fields omitted for now
}

#[derive(Deserialize, Debug)]
pub struct Tracks<'dt> {
    #[serde(borrow)]
    pub track: Option<Vec<Track<'dt>>>,
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
pub struct GetInfo<'dt> {
    pub name: &'dt str,
    pub artist: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Url<'dt>,
    pub image: Vec<Image<'dt>>,
    #[serde(deserialize_with = "str_to_val")]
    pub listeners: u32,
    #[serde(deserialize_with = "str_to_val")]
    pub playcount: u32,
    #[serde(default)]
    #[serde(deserialize_with = "str_to_option")]
    pub userplaycount: Option<u32>,
    pub tracks: Tracks<'dt>,
    pub tags: Tags<'dt>,
}

lastfm_t!(
    album,
    GetInfo,
    _Info,
    Method,
    AlbumGetInfo,
    Params,
    GetInfo,
    [
        artist: &'rq str,
        album: &'rq str,
        mbid: Option<&'rq str>,
        autocorrect: Option<u32>,
        lang: Option<&'rq str>,
        username: Option<&'rq str>
    ]
);

// ----------------------------------------------------------------

pub type GetTags<'dt> = Tags<'dt>;

lastfm_t!(
    tags,
    GetTags,
    _UserTags,
    Method,
    AlbumGetTags,
    Params,
    GetTags,
    [
        artist: &'rq str,
        album: &'rq str,
        mbid: Option<&'rq str>,
        autocorrect: Option<u32>,
        user: Option<&'rq str>
    ]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Tag2<'dt> {
    pub name: &'dt str,
    #[serde(default)]
    #[serde(deserialize_with = "str_to_option")]
    pub count: Option<u32>,
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
    Method,
    AlbumGetTopTags,
    Params,
    GetTopTags,
    [
        artist: &'rq str,
        album: &'rq str,
        mbid: Option<&'rq str>,
        autocorrect: Option<u32>
    ]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Album<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub artist: &'dt str,
    pub url: Url<'dt>,
    pub image: Vec<Image<'dt>>,
    #[serde(default)]
    #[serde(deserialize_with = "str_to_option")]
    pub streamable: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct SearchData<'dt> {
    #[serde(borrow)]
    pub album: Option<Vec<Album<'dt>>>,
}

opensearch_t!(
    results,
    Search,
    _Search,
    albummatches,
    SearchData,
    Method,
    AlbumSearch,
    Params,
    Search,
    [album: &'rq str, limit: Option<u32>, page: Option<u32>]
);

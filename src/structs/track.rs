#![allow(non_snake_case)]

use std::convert::Into;
use std::marker::PhantomData;

use url::Url as StdUrl;

use lastfm_type::{LastfmType, Request, RequestParams};
use super::common::{Url, Image, SearchQuery, str_to_option, str_to_val};

// ----------------------------------------------------------------

#[derive(Debug)]
pub enum Params<'pr> {
    AddTags {
        artist: &'pr str,
        track: &'pr str,
        tags: &'pr str,
    },
    GetCorrection { artist: &'pr str, track: &'pr str },
    GetInfo {
        artist: &'pr str,
        track: &'pr str,
        mbid: Option<&'pr str>,
        autocorrect: Option<u32>,
        username: Option<&'pr str>,
    },
    GetSimilar {
        artist: &'pr str,
        track: &'pr str,
        mbid: Option<&'pr str>,
        autocorrect: Option<u32>,
        limit: Option<u32>,
    },
    GetTags {
        artist: &'pr str,
        track: &'pr str,
        mbid: Option<&'pr str>,
        autocorrect: Option<u32>,
        user: Option<&'pr str>,
    },
    GetTopTags {
        artist: &'pr str,
        track: &'pr str,
        mbid: Option<&'pr str>,
        autocorrect: Option<u32>,
    },
    Love {
        artist: &'pr str,
        track: &'pr str,
    },
    RemoveTag {
        artist: &'pr str,
        track: &'pr str,
        tag: &'pr str,
    },
    Scrobble, // auth, also has batch form
    Search {
        artist: &'pr str,
        track: &'pr str,
        limit: Option<u32>,
        page: Option<u32>,
    },
    Unlove {
        artist: &'pr str,
        track: &'pr str,
    },
    UpdateNowPlaying {
        artist: &'pr str,
        track: &'pr str,
        album: Option<&'pr str>,
        trackNumber: Option<u32>,
        context: Option<&'pr str>,
        mbid: Option<&'pr str>,
        duration: Option<u32>,
        albumArtist: Option<&'pr str>,
    },
}

impl<'pr> RequestParams for Params<'pr> {
    fn method(&self) -> &str {
        match *self {
            Params::AddTags { .. } => "track.addtags",
            Params::GetCorrection { .. } => "track.getcorrection",
            Params::GetInfo { .. } => "track.getinfo",
            Params::GetSimilar { .. } => "track.getsimilar",
            Params::GetTags { .. } => "track.gettags",
            Params::GetTopTags { .. } => "track.gettoptags",
            Params::Love { .. } => "track.love",
            Params::RemoveTag { .. } => "track.removetags",
            Params::Scrobble { .. } => "track.scrobble",
            Params::Search { .. } => "track.search",
            Params::Unlove { .. } => "track.unlove",
            Params::UpdateNowPlaying { .. } => "track.updatenowplaying",
        }
    }

    fn needs_signature(&self) -> bool {
        match *self {
            Params::AddTags { .. } => true,
            Params::Love { .. } => true,
            Params::RemoveTag { .. } => true,
            Params::Scrobble { .. } => true,
            Params::Unlove { .. } => true,
            Params::UpdateNowPlaying { .. } => true,
            _ => false,
        }
    }

    fn needs_session_key(&self) -> bool {
        self.needs_signature()
    }

    fn append_to(&self, url: &mut StdUrl) {
        let mut query = url.query_pairs_mut();
        match *self {
            Params::AddTags {
                artist,
                track,
                tags,
            } => {
                query.append_pair("artist", artist);
                query.append_pair("track", track);
                query.append_pair("tags", tags);
            }
            Params::GetCorrection { artist, track } => {
                query.append_pair("artist", artist);
                query.append_pair("track", track);
            }
            Params::GetInfo {
                artist,
                track,
                mbid,
                autocorrect,
                username,
            } => {
                query.append_pair("artist", artist);
                query.append_pair("track", track);
                if let Some(mbid) = mbid {
                    query.append_pair("mbid", mbid);
                }
                if let Some(autocorrect) = autocorrect {
                    query.append_pair("autocorrect", &autocorrect.to_string());
                }
                if let Some(username) = username {
                    query.append_pair("username", username);
                }
            }
            Params::GetSimilar {
                artist,
                track,
                mbid,
                autocorrect,
                limit,
            } => {
                query.append_pair("artist", artist);
                query.append_pair("track", track);
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
                track,
                mbid,
                autocorrect,
                user,
            } => {
                query.append_pair("artist", artist);
                query.append_pair("track", track);
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
                track,
                mbid,
                autocorrect,
            } => {
                query.append_pair("artist", artist);
                query.append_pair("track", track);
                if let Some(mbid) = mbid {
                    query.append_pair("mbid", mbid);
                }
                if let Some(autocorrect) = autocorrect {
                    query.append_pair("autocorrect", &autocorrect.to_string());
                }
            }
            Params::Love { artist, track } => {
                query.append_pair("artist", artist);
                query.append_pair("track", track);
            }
            Params::RemoveTag { artist, track, tag } => {
                query.append_pair("artist", artist);
                query.append_pair("track", track);
                query.append_pair("tag", tag);
            }
            Params::Scrobble => {}
            Params::Search {
                artist,
                track,
                limit,
                page,
            } => {
                query.append_pair("artist", artist);
                query.append_pair("track", track);
                if let Some(limit) = limit {
                    query.append_pair("limit", &limit.to_string());
                }
                if let Some(page) = page {
                    query.append_pair("page", &page.to_string());
                }
            }
            Params::Unlove { artist, track } => {
                query.append_pair("artist", artist);
                query.append_pair("track", track);
            }
            Params::UpdateNowPlaying {
                artist,
                track,
                album,
                trackNumber,
                context,
                mbid,
                duration,
                albumArtist,
            } => {
                params!(query,
                    [
                        artist: artist,
                        track: track
                    ],
                    [
                        album: album,
                        trackNumber: cv!(trackNumber),
                        context: context,
                        mbid: mbid,
                        duration: cv!(duration),
                        albumArtist: albumArtist
                    ]
                );
            }
        }
    }
}

// ----------------------------------------------------------------

empty_lastfm_t!(
    AddTags,
    Params,
    AddTags,
    [
        artist: &'rq str,
        track: &'rq str,
        tags: &'rq str
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
    pub url: Url<'dt>,
    pub artist: Artist<'dt>,
}

#[derive(Deserialize, Debug)]
pub struct Correction<'dt> {
    #[serde(borrow)]
    pub track: Track<'dt>,
}

#[derive(Deserialize, Debug)]
pub struct GetCorrections<'dt> {
    #[serde(borrow)]
    correction: Correction<'dt>,
}

lastfm_t!(
    corrections,
    GetCorrections,
    _Correction,
    Params,
    GetCorrection,
    [artist: &'rq str, track: &'rq str]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Album<'dt> {
    pub artist: &'dt str,
    pub title: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Url<'dt>,
    pub image: Vec<Image<'dt>>,
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
    pub mbid: Option<&'dt str>,
    pub url: Url<'dt>,
    #[serde(deserialize_with = "str_to_val")]
    pub duration: u32,
    #[serde(deserialize_with = "str_to_val")]
    pub listeners: u32,
    #[serde(default)]
    #[serde(deserialize_with = "str_to_option")]
    pub playcount: Option<u32>,
    #[serde(default)]
    #[serde(deserialize_with = "str_to_option")]
    pub userplaycount: Option<u32>,
    #[serde(default)]
    #[serde(deserialize_with = "str_to_option")]
    pub userloved: Option<u32>,
    pub artist: Artist<'dt>,
    pub album: Album<'dt>,
    pub toptags: Tags<'dt>,
}

lastfm_t!(
    track,
    GetInfo,
    _Info,
    Params,
    GetInfo,
    [
        artist: &'rq str,
        track: &'rq str,
        mbid: Option<&'rq str>,
        autocorrect: Option<u32>,
        username: Option<&'rq str>
    ]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Similar<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Url<'dt>,
    #[serde(deserialize_with = "str_to_option")]
    pub playcount: Option<u32>,
    #[serde(rename = "match")]
    pub trackmatch: f32,
    pub duration: u32,
    pub artist: Artist<'dt>,
    pub image: Vec<Image<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct GetSimilar<'dt> {
    #[serde(borrow)]
    pub track: Option<Vec<Similar<'dt>>>,
}

lastfm_t!(
    similartracks,
    GetSimilar,
    _SimilarList,
    Params,
    GetSimilar,
    [
        artist: &'rq str,
        track: &'rq str,
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
        track: &'rq str,
        mbid: Option<&'rq str>,
        autocorrect: Option<u32>,
        user: Option<&'rq str>
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
        track: &'rq str,
        mbid: Option<&'rq str>,
        autocorrect: Option<u32>
    ]
);

// ----------------------------------------------------------------

empty_lastfm_t!(
    Love,
    Params,
    Love,
    [
        artist: &'rq str,
        track: &'rq str
    ]
);

// ----------------------------------------------------------------

empty_lastfm_t!(
    RemoveTag,
    Params,
    RemoveTag,
    [
        artist: &'rq str,
        track: &'rq str,
        tag: &'rq str
    ]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Track2<'dt> {
    pub name: &'dt str,
    pub artist: &'dt str,
    pub url: Url<'dt>,
    #[serde(deserialize_with = "str_to_val")]
    pub listeners: u32,
    pub image: Vec<Image<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct SearchData<'dt> {
    #[serde(borrow)]
    pub track: Option<Vec<Track2<'dt>>>,
}

opensearch_t!(
    results,
    Search,
    _Search,
    trackmatches,
    SearchData,
    Params,
    Search,
    [
        artist: &'rq str,
        track: &'rq str,
        limit: Option<u32>,
        page: Option<u32>
    ]
);

// ----------------------------------------------------------------

empty_lastfm_t!(
    Unlove,
    Params,
    Unlove,
    [
        artist: &'rq str,
        track: &'rq str
    ]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct UpdateNowPlaying<'dt> {
    pub track: &'dt str,
    pub fail_this: &'dt str,
}

lastfm_t!(
    nowplaying,
    UpdateNowPlaying,
    _UpdateNowPlaying,
    Params,
    UpdateNowPlaying,
    [
        artist: &'rq str,
        track: &'rq str,
        album: Option<&'rq str>,
        trackNumber: Option<u32>,
        context: Option<&'rq str>,
        mbid: Option<&'rq str>,
        duration: Option<u32>,
        albumArtist: Option<&'rq str>
    ]
);

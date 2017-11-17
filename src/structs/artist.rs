use std::convert::Into;
use std::marker::PhantomData;

use url::Url as StdUrl;

use methods::Method;
use lastfm_type::{LastfmType, Request, RequestParams};
use super::common::{Url, Image, SearchQuery, str_to_option};

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
    }
}

impl<'pr> RequestParams for Params<'pr> {
    fn append_to(&self, url: &mut StdUrl) {
        let mut query = url.query_pairs_mut();
        match *self {
            Params::AddTags { artist, tags } => {
                query.append_pair("artist", artist);
                query.append_pair("tags", tags);
            },
            Params::GetCorrection { artist } => {
                query.append_pair("artist", artist);
            },
            Params::GetInfo { artist, mbid, autocorrect, lang, username } => {
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
            },
            Params::GetSimilar { artist, mbid, autocorrect, limit } => {
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
            },
            Params::GetTags { artist, mbid, autocorrect, user } => {
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
            },
            Params::GetTopAlbums { artist, mbid, autocorrect, limit, page } => {
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
            },
            Params::GetTopTags { artist, mbid, autocorrect } => {
                query.append_pair("artist", artist);
                if let Some(mbid) = mbid {
                    query.append_pair("mbid", mbid);
                }
                if let Some(autocorrect) = autocorrect {
                    query.append_pair("autocorrect", &autocorrect.to_string());
                }
            },
            Params::GetTopTracks { artist, mbid, autocorrect, limit, page } => {
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
            },
            Params::RemoveTag { artist, tag } => {
                query.append_pair("artist", artist);
                query.append_pair("tag", tag);
            },
            Params::Search { artist, limit, page } => {
                query.append_pair("artist", artist);
                if let Some(limit) = limit {
                    query.append_pair("limit", &limit.to_string());
                }
                if let Some(page) = page {
                    query.append_pair("page", &page.to_string());
                }
            },
        }
    }
}

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Artist1<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Option<Url>,
}

#[derive(Deserialize, Debug)]
pub struct Correction<'dt> {
    #[serde(borrow)]
    pub artist: Option<Artist1<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct Corrections<'dt> {
    #[serde(borrow)]
    pub correction: Option<Correction<'dt>>,
}

lastfm_t!(
    corrections,
    Corrections,
    _Corrections,
    Method,
    ArtistGetCorrection,
    Params,
    GetCorrection,
    [artist: &'rq str]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Stats<'dt> {
    #[serde(skip)]
    phantom: PhantomData<&'dt ()>,
    #[serde(deserialize_with="str_to_option")]
    pub listeners: Option<u32>,
    #[serde(deserialize_with="str_to_option")]
    pub playcount: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct Similar1<'dt> {
    pub name: &'dt str,
    pub url: Option<Url>,
    #[serde(borrow)]
    pub image: Option<Vec<Image<'dt>>>,
}

#[derive(Deserialize, Debug)]
pub struct SimilarList1<'dt> {
    #[serde(borrow)]
    pub artist: Option<Vec<Similar1<'dt>>>,
}

#[derive(Deserialize, Debug)]
pub struct Tag1<'dt> {
    pub name: &'dt str,
    pub url: Option<Url>,
}

#[derive(Deserialize, Debug)]
pub struct Tags<'dt> {
    #[serde(borrow)]
    pub tag: Option<Vec<Tag1<'dt>>>,
}

#[derive(Deserialize, Debug)]
pub struct Link<'dt> {
    #[serde(rename="#text")]
    pub text: &'dt str,
    pub rel: Option<&'dt str>,
    pub href: Option<Url>,
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
    pub published: Option<&'dt str>, // TODO: change type to actual date (like Url)
    pub summary: Option<String>, // may have escape sequences
    pub content: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Info<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Option<Url>,
    pub image: Option<Vec<Image<'dt>>>,
    #[serde(deserialize_with="str_to_option")]
    pub streamable: Option<u32>,
    #[serde(deserialize_with="str_to_option")]
    pub ontour: Option<u32>,
    pub stats: Stats<'dt>,
    #[serde(borrow)]
    pub similar: SimilarList1<'dt>,
    #[serde(borrow)]
    pub tags: Tags<'dt>,
    pub bio: Bio<'dt>,
}

lastfm_t!(
    artist,
    Info,
    _Info,
    Method,
    ArtistGetInfo,
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
    #[serde(rename="match")]
    #[serde(deserialize_with="str_to_option")]
    pub similar_match: Option<f32>,
    pub url: Option<Url>,
    #[serde(borrow)]
    pub image: Option<Vec<Image<'dt>>>,
    #[serde(deserialize_with="str_to_option")]
    pub streamable: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct SimilarList<'dt> {
    #[serde(borrow)]
    pub artist: Option<Vec<Similar2<'dt>>>,
}

lastfm_t!(
    similarartists,
    SimilarList,
    _SimilarList,
    Method,
    ArtistGetSimilar,
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

pub type UserTags<'dt> = Tags<'dt>;

lastfm_t!(
    tags,
    UserTags,
    _UserTags,
    Method,
    ArtistGetTags,
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
    #[serde(deserialize_with="str_to_option")]
    pub playcount: Option<u32>,
    pub mbid: Option<&'dt str>,
    pub url: Option<Url>,
    #[serde(borrow)]
    pub artist: Option<Artist1<'dt>>,
    #[serde(borrow)]
    pub image: Option<Vec<Image<'dt>>>,
}

#[derive(Deserialize, Debug)]
pub struct TopAlbums<'dt> {
    #[serde(borrow)]
    pub album: Option<Vec<Album<'dt>>>,
}

lastfm_t!(
    topalbums,
    TopAlbums,
    _TopAlbums,
    Method,
    ArtistGetTopAlbums,
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
    #[serde(deserialize_with="str_to_option")]
    pub count: Option<u32>,
    pub url: Option<Url>,
}

#[derive(Deserialize, Debug)]
pub struct TopTags<'dt> {
    #[serde(borrow)]
    pub tag: Option<Vec<Tag2<'dt>>>,
}

lastfm_t!(
    toptags,
    TopTags,
    _TopTags,
    Method,
    ArtistGetTopTags,
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
    pub url: Option<Url>,
}

#[derive(Deserialize, Debug)]
pub struct Track<'dt> {
    pub name: &'dt str,
    #[serde(deserialize_with="str_to_option")]
    pub playcount: Option<u32>,
    #[serde(deserialize_with="str_to_option")]
    pub listeners: Option<u32>,
    pub mbid: Option<&'dt str>,
    pub url: Option<Url>,
    #[serde(deserialize_with="str_to_option")]
    pub streamable: Option<u32>,
    #[serde(borrow)]
    pub artist: Option<Artist<'dt>>,
    #[serde(borrow)]
    pub image: Option<Vec<Image<'dt>>>,
}

#[derive(Deserialize, Debug)]
pub struct TopTracks<'dt> {
    #[serde(borrow)]
    pub track: Option<Vec<Track<'dt>>>,
}

lastfm_t!(
    toptracks,
    TopTracks,
    _TopTracks,
    Method,
    ArtistGetTopTracks,
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
    #[serde(deserialize_with="str_to_option")]
    pub listeners: Option<u32>,
    pub mbid: Option<&'dt str>,
    pub url: Option<Url>,
    #[serde(deserialize_with="str_to_option")]
    pub streamable: Option<u32>,
    #[serde(borrow)]
    pub image: Option<Vec<Image<'dt>>>,
}

#[derive(Deserialize, Debug)]
pub struct SearchData<'dt> {
    #[serde(borrow)]
    pub artist: Option<Vec<Artist2<'dt>>>
}

opensearch_t!(
    results,
    Search,
    _Search,
    artistmatches,
    SearchData,
    Method,
    ArtistSearch,
    Params,
    Search,
    [
        artist: &'rq str,
        limit: Option<u32>,
        page: Option<u32>
    ]
);
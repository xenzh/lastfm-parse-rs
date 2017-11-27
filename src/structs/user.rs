use std::convert::Into;
use std::marker::PhantomData;

use url::Url as StdUrl;

use methods::Method;
use lastfm_type::{LastfmType, Request, RequestParams};
use super::common::{Url, Image, str_to_option, str_to_val};

// ----------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub enum TaggingType {
    Artist,
    Album,
    Track,
}

impl TaggingType {
    pub fn to_str(&self) -> &str {
        match *self {
            TaggingType::Artist => "artist",
            TaggingType::Album => "album",
            TaggingType::Track => "track",
        }
    }
}

pub type UnixTimestamp = i64;
pub type ChartDate = i64;

#[derive(Debug, Clone, Copy)]
pub enum Period {
    OneWeek,
    OneMonth,
    ThreeMonth,
    SixMonth,
    OneYear,
    Overall,
}

impl Period {
    pub fn to_str(&self) -> &str {
        match *self {
            Period::OneWeek => "7day",
            Period::OneMonth => "1month",
            Period::ThreeMonth => "3month",
            Period::SixMonth => "6month",
            Period::OneYear => "12month",
            Period::Overall => "overall",
        }
    }
}

// ----------------------------------------------------------------

#[derive(Debug)]
pub enum Params<'pr> {
    GetArtistTracks {
        user: &'pr str,
        artist: &'pr str,
        start: Option<UnixTimestamp>,
        end: Option<UnixTimestamp>,
        page: Option<u32>,
    },
    GetFriends {
        user: &'pr str,
        recenttracks: Option<bool>,
        limit: Option<u32>,
        page: Option<u32>,
    },
    GetInfo { user: &'pr str },
    GetLovedTracks {
        user: &'pr str,
        limit: Option<u32>,
        page: Option<u32>,
    },
    GetPersonalTags {
        user: &'pr str,
        tag: &'pr str,
        taggingtype: TaggingType,
        limit: Option<u32>,
        page: Option<u32>,
    },
    GetRecentTracks {
        user: &'pr str,
        extended: Option<bool>,
        from: Option<UnixTimestamp>,
        to: Option<UnixTimestamp>,
        limit: Option<u32>,
        page: Option<u32>,
    },
    GetTopAlbums {
        user: &'pr str,
        period: Option<Period>,
        limit: Option<u32>,
        page: Option<u32>,
    },
    GetTopArtists {
        user: &'pr str,
        period: Option<Period>,
        limit: Option<u32>,
        page: Option<u32>,
    },
    GetTopTags {
        user: &'pr str,
        limit: Option<u32>
    },
    GetTopTracks {
        user: &'pr str,
        period: Option<Period>,
        limit: Option<u32>,
        page: Option<u32>,
    },
    GetWeeklyAlbumChart {
        user: &'pr str,
        from: Option<ChartDate>,
        to: Option<ChartDate>,
    },
    GetWeeklyArtistChart {
        user: &'pr str,
        from: Option<ChartDate>,
        to: Option<ChartDate>,
    },
    GetWeeklyChartList { user: &'pr str },
    GetWeeklyTrackChart {
        user: &'pr str,
        from: Option<ChartDate>,
        to: Option<ChartDate>,
    },
}

impl<'pr> RequestParams for Params<'pr> {
    fn append_to(&self, url: &mut StdUrl) {
        let mut query = url.query_pairs_mut();
        match *self {
            Params::GetArtistTracks { user, artist, start, end, page } => {
                query.append_pair("user", user);
                query.append_pair("artist", artist);
                if let Some(start) = start {
                    query.append_pair("start", &start.to_string());
                }
                if let Some(end) = end {
                    query.append_pair("end", &end.to_string());
                }
                if let Some(page) = page {
                    query.append_pair("page", &page.to_string());
                }
            },
            Params::GetFriends { user, recenttracks, limit, page } => {
                query.append_pair("user", user);
                if let Some(recenttracks) = recenttracks {
                    query.append_pair("recenttracks", &(recenttracks as u32).to_string());
                }
                if let Some(limit) = limit {
                    query.append_pair("limit", &limit.to_string());
                }
                if let Some(page) = page {
                    query.append_pair("page", &page.to_string());
                }
            },
            Params::GetInfo { user } => {
                query.append_pair("user", user);
            },
            Params::GetLovedTracks { user, limit, page } => {
                query.append_pair("user", user);
                if let Some(limit) = limit {
                    query.append_pair("limit", &limit.to_string());
                }
                if let Some(page) = page {
                    query.append_pair("page", &page.to_string());
                }
            },
            Params::GetPersonalTags { user, tag, taggingtype, limit, page } => {
                query.append_pair("user", user);
                query.append_pair("tag", tag);
                query.append_pair("taggingtype", taggingtype.to_str());
                if let Some(limit) = limit {
                    query.append_pair("limit", &limit.to_string());
                }
                if let Some(page) = page {
                    query.append_pair("page", &page.to_string());
                }
            },
            Params::GetRecentTracks { user, extended, from, to, limit, page } => {
                query.append_pair("user", user);
                if let Some(extended) = extended {
                    query.append_pair("extended", &(extended as u32).to_string());
                }
                if let Some(from) = from {
                    query.append_pair("from", &from.to_string());
                }
                if let Some(to) = to {
                    query.append_pair("to", &to.to_string());
                }
                if let Some(limit) = limit {
                    query.append_pair("limit", &limit.to_string());
                }
                if let Some(page) = page {
                    query.append_pair("page", &page.to_string());
                }
            },
            Params::GetTopAlbums { user, period, limit, page } => {
                query.append_pair("user", user);
                if let Some(period) = period {
                    query.append_pair("period", period.to_str());
                }
                if let Some(limit) = limit {
                    query.append_pair("limit", &limit.to_string());
                }
                if let Some(page) = page {
                    query.append_pair("page", &page.to_string());
                }
            },
            Params::GetTopArtists { user, period, limit, page } => {
                query.append_pair("user", user);
                if let Some(period) = period {
                    query.append_pair("period", period.to_str());
                }
                if let Some(limit) = limit {
                    query.append_pair("limit", &limit.to_string());
                }
                if let Some(page) = page {
                    query.append_pair("page", &page.to_string());
                }
            },
            Params::GetTopTags { user, limit } => {
                query.append_pair("user", user);
                if let Some(limit) = limit {
                    query.append_pair("limit", &limit.to_string());
                }
            },
            Params::GetTopTracks { user, period, limit, page } => {
                query.append_pair("user", user);
                if let Some(period) = period {
                    query.append_pair("period", period.to_str());
                }
                if let Some(limit) = limit {
                    query.append_pair("limit", &limit.to_string());
                }
                if let Some(page) = page {
                    query.append_pair("page", &page.to_string());
                }
            },
            Params::GetWeeklyAlbumChart { user, from, to } => {
                query.append_pair("user", user);
                if let Some(from) = from {
                    query.append_pair("from", &from.to_string());
                }
                if let Some(to) = to {
                    query.append_pair("to", &to.to_string());
                }
            },
            Params::GetWeeklyArtistChart { user, from, to } => {
                query.append_pair("user", user);
                if let Some(from) = from {
                    query.append_pair("from", &from.to_string());
                }
                if let Some(to) = to {
                    query.append_pair("to", &to.to_string());
                }
            },
            Params::GetWeeklyChartList { user } => {
                query.append_pair("user", user);
            },
            Params::GetWeeklyTrackChart { user, from, to } => {
                query.append_pair("user", user);
                if let Some(from) = from {
                    query.append_pair("from", &from.to_string());
                }
                if let Some(to) = to {
                    query.append_pair("to", &to.to_string());
                }
            },
        }
    }
}

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Id1<'dt> {
    #[serde(rename="#text")]
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
}

#[derive(Deserialize, Debug)]
pub struct Id2<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Option<Url<'dt>>,
    pub image: Option<Vec<Image<'dt>>>,
}

#[derive(Deserialize, Debug)]
pub struct Date1<'dt> {
    #[serde(deserialize_with="str_to_option")]
    pub uts: Option<UnixTimestamp>,
    #[serde(rename="#text")]
    pub text: &'dt str,
}

#[derive(Deserialize, Debug)]
pub struct NowPlaying<'dt> {
    #[serde(skip)]
    phantom: PhantomData<& 'dt ()>,
    #[serde(deserialize_with="str_to_val")]
    pub nowplaying: bool,
}

#[derive(Deserialize, Debug)]
pub struct Track1<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    #[serde(deserialize_with="str_to_option")]
    pub streamable: Option<u32>,
    // #[serde(deserialize_with="str_to_option")]
    // pub loved: Option<u32>,
    pub artist: Option<Id1<'dt>>,
    pub album: Option<Id1<'dt>>,
    pub url: Option<Url<'dt>>,
    pub image: Option<Vec<Image<'dt>>>,
    pub date: Option<Date1<'dt>>,
    #[serde(rename="@attr")]
    pub now: Option<NowPlaying<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct GetArtistTracks<'dt> {
    #[serde(borrow)]
    pub track: Option<Vec<Track1<'dt>>>,
}

lastfm_t!(
    artisttracks,
    GetArtistTracks,
    _ArtistTracks,
    Method,
    UserGetArtistTracks,
    Params,
    GetArtistTracks,
    [
        user: &'rq str,
        artist: &'rq str,
        start: Option<UnixTimestamp>,
        end: Option<UnixTimestamp>,
        page: Option<u32>
    ]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub enum Gender {
    #[serde(rename="m")]
    Male,
    #[serde(rename="f")]
    Female,
    #[serde(rename="n")]
    NotSpecified,
}

#[derive(Deserialize, Debug)]
pub struct Date2<'dt> {
    #[serde(deserialize_with="str_to_val")]
    pub unixtime: UnixTimestamp,
    #[serde(rename="#text")]
    pub text: &'dt str,
}

#[derive(Deserialize, Debug)]
pub struct Date3<'dt> {
    #[serde(deserialize_with="str_to_option")]
    pub uts: Option<UnixTimestamp>,
    pub date: &'dt str,
}

#[derive(Deserialize, Debug)]
pub struct Track2<'dt> {
    pub name: &'dt str,
    pub artist: Option<Id2<'dt>>,
    pub album: Option<Id2<'dt>>,
    pub mbid: Option<&'dt str>,
    pub url: Option<Url<'dt>>,
    #[serde(rename="@attr")]
    pub date: Option<Date3<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct User<'dt> {
    pub name: &'dt str,
    pub url: Option<Url<'dt>>,
    pub country: &'dt str,
    #[serde(deserialize_with="str_to_option")]
    pub age: Option<u32>,
    pub gender: Gender,
    #[serde(deserialize_with="str_to_option")]
    pub playcount: Option<u32>,
    #[serde(deserialize_with="str_to_option")]
    pub playlists: Option<u32>,
    #[serde(deserialize_with="str_to_option")]
    pub bootstrap: Option<u32>,
    pub registered: Date2<'dt>,
    pub image: Option<Vec<Image<'dt>>>,
    pub recenttrack: Option<Track2<'dt>>,

    // Service always returns FIXME in fields below
    pub subscriber: Option<&'dt str>,
    #[serde(rename="type")]
    pub u_type: Option<&'dt str>,
    pub scrobblesource: Option<&'dt str>,
}

#[derive(Deserialize, Debug)]
pub struct GetFriends<'dt> {
    #[serde(borrow)]
    pub user: Option<Vec<User<'dt>>>,
}

lastfm_t!(
    friends,
    GetFriends,
    _Friends,
    Method,
    UserGetFriends,
    Params,
    GetFriends,
    [
        user: &'rq str,
        recenttracks: Option<bool>,
        limit: Option<u32>,
        page: Option<u32>
    ]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Date4<'dt> {
    #[serde(skip)]
    phantom: PhantomData<&'dt ()>,
    #[serde(deserialize_with="str_to_val")]
    pub unixtime: UnixTimestamp,
    #[serde(rename="#text")]
    pub text: UnixTimestamp,
}

#[derive(Deserialize, Debug)]
pub struct GetInfo<'dt> {
    pub name: &'dt str,
    pub url: Option<Url<'dt>>,
    pub country: &'dt str,
    #[serde(deserialize_with="str_to_option")]
    pub age: Option<u32>,
    pub gender: Gender,
    #[serde(deserialize_with="str_to_option")]
    pub playcount: Option<u32>,
    #[serde(deserialize_with="str_to_option")]
    pub playlists: Option<u32>,
    #[serde(deserialize_with="str_to_option")]
    pub bootstrap: Option<u32>,
    pub registered: Date4<'dt>,
    pub image: Option<Vec<Image<'dt>>>,
    pub recenttrack: Option<Track2<'dt>>,

    // Service always returns FIXME in fields below
    pub subscriber: Option<&'dt str>,
    #[serde(rename="type")]
    pub u_type: Option<&'dt str>,
    pub scrobblesource: Option<&'dt str>,
}

lastfm_t!(
    user,
    GetInfo,
    _Info,
    Method,
    UserGetInfo,
    Params,
    GetInfo,
    [ user: &'rq str ]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Streamable<'dt> {
    #[serde(skip)]
    phantom: PhantomData<&'dt ()>,
    #[serde(rename="#text")]
    #[serde(deserialize_with="str_to_val")]
    pub streamable: u32,
    #[serde(deserialize_with="str_to_val")]
    pub fulltrack: u32,
}

#[derive(Deserialize, Debug)]
pub struct Track3<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Option<Url<'dt>>,
    pub date: Date1<'dt>,
    pub artist: Option<Id2<'dt>>,
    pub image: Option<Vec<Image<'dt>>>,
    pub streamable: Option<Streamable<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct GetLovedTracks<'dt> {
    #[serde(borrow)]
    pub track: Option<Vec<Track3<'dt>>>,
}

lastfm_t!(
    lovedtracks,
    GetLovedTracks,
    _LovedTracks,
    Method,
    UserGetLovedTracks,
    Params,
    GetLovedTracks,
    [
        user: &'rq str,
        limit: Option<u32>,
        page: Option<u32>
    ]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Artist1<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Option<Url<'dt>>,
    #[serde(deserialize_with="str_to_option")]
    pub streamable: Option<u32>,
    pub image: Option<Vec<Image<'dt>>>,
}

#[derive(Deserialize, Debug)]
pub struct ArtistTaggings<'dt> {
    #[serde(borrow)]
    pub artist: Option<Vec<Artist1<'dt>>>,
}

#[derive(Deserialize, Debug)]
pub struct Track4<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Option<Url<'dt>>,
    pub streamable: Option<Streamable<'dt>>,
    pub artist: Id2<'dt>,
    pub image: Option<Vec<Image<'dt>>>,

    // Service always returns FIXME in fields below
    pub duration: Option<&'dt str>,
}

#[derive(Deserialize, Debug)]
pub struct TrackTaggings<'dt> {
    #[serde(borrow)]
    pub track: Option<Vec<Track4<'dt>>>,
}

#[derive(Deserialize, Debug)]
pub struct Album1<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Option<Url<'dt>>,
    pub artist: Id2<'dt>,
}

#[derive(Deserialize, Debug)]
pub struct AlbumTaggings<'dt> {
    #[serde(borrow)]
    pub album: Option<Vec<Album1<'dt>>>,
}

#[derive(Deserialize, Debug)]
pub struct GetTaggings<'dt> {
    #[serde(borrow)]
    pub artists: Option<ArtistTaggings<'dt>>,
    #[serde(borrow)]
    pub tracks: Option<TrackTaggings<'dt>>,
    #[serde(borrow)]
    pub albums: Option<AlbumTaggings<'dt>>,
}

lastfm_t!(
    taggings,
    GetTaggings,
    _Taggings,
    Method,
    UserGetPersonalTags,
    Params,
    GetPersonalTags,
    [
        user: &'rq str,
        tag: &'rq str,
        taggingtype: TaggingType,
        limit: Option<u32>,
        page: Option<u32>
    ]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Track5<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    #[serde(deserialize_with="str_to_option")]
    pub streamable: Option<u32>,
    #[serde(deserialize_with="str_to_option")]
    pub loved: Option<u32>,
    pub artist: Option<Id2<'dt>>,
    pub album: Option<Id1<'dt>>,
    pub url: Option<Url<'dt>>,
    pub image: Option<Vec<Image<'dt>>>,
    pub date: Option<Date1<'dt>>,
    #[serde(rename="@attr")]
    pub now: Option<NowPlaying<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct GetRecentTracks<'dt> {
    #[serde(borrow)]
    pub track: Option<Vec<Track5<'dt>>>,
}

lastfm_t!(
    recenttracks,
    GetRecentTracks,
    _RecentTracks,
    Method,
    UserGetRecentTracks,
    Params,
    GetRecentTracks,
    [
        user: &'rq str,
        extended: Option<bool>,
        from: Option<UnixTimestamp>,
        to: Option<UnixTimestamp>,
        limit: Option<u32>,
        page: Option<u32>
    ]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Rank<'dt> {
    #[serde(skip)]
    phantom: PhantomData<&'dt ()>,
    #[serde(deserialize_with="str_to_val")]
    pub rank: u32,
}

#[derive(Deserialize, Debug)]
pub struct Album2<'dt> {
    pub name: &'dt str,
    #[serde(deserialize_with="str_to_option")]
    pub playcount: Option<u32>,
    pub mbid: Option<&'dt str>,
    pub url: Option<Url<'dt>>,
    pub artist: Id2<'dt>,
    pub image: Option<Vec<Image<'dt>>>,
    #[serde(rename="@attr")]
    pub rank: Option<Rank<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct GetTopAlbums<'dt> {
    #[serde(borrow)]
    pub album: Option<Vec<Album2<'dt>>>,
}

lastfm_t!(
    topalbums,
    GetTopAlbums,
    _GetTopAlbums,
    Method,
    UserGetTopAlbums,
    Params,
    GetTopAlbums,
    [
        user: &'rq str,
        period: Option<Period>,
        limit: Option<u32>,
        page: Option<u32>
    ]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Artist2<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Option<Url<'dt>>,
    #[serde(deserialize_with="str_to_option")]
    pub playcount: Option<u32>,
    #[serde(deserialize_with="str_to_option")]
    pub streamable: Option<u32>,
    pub image: Option<Vec<Image<'dt>>>,
    #[serde(rename="@attr")]
    pub rank: Option<Rank<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct GetTopArtists<'dt> {
    #[serde(borrow)]
    pub artist: Option<Vec<Artist2<'dt>>>,
}

lastfm_t!(
    topartists,
    GetTopArtists,
    _GetTopArtists,
    Method,
    UserGetTopArtists,
    Params,
    GetTopArtists,
    [
        user: &'rq str,
        period: Option<Period>,
        limit: Option<u32>,
        page: Option<u32>
    ]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Tag<'dt> {
    pub name: &'dt str,
    #[serde(deserialize_with="str_to_val")]
    pub count: u32,
    pub url: Option<Url<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct GetTopTags<'dt> {
    #[serde(borrow)]
    pub tag: Option<Vec<Tag<'dt>>>,
}

lastfm_t!(
    toptags,
    GetTopTags,
    _GetTopTags,
    Method,
    UserGetTopTags,
    Params,
    GetTopTags,
    [
        user: &'rq str,
        limit: Option<u32>
    ]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Track6<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Option<&'dt str>,
    #[serde(deserialize_with="str_to_val")]
    pub playcount: u32,
    #[serde(deserialize_with="str_to_option")]
    pub duration: Option<u32>,
    pub streamable: Option<Streamable<'dt>>,
    pub artist: Option<Id2<'dt>>,
    pub image: Option<Vec<Image<'dt>>>,
    #[serde(rename="@attr")]
    pub rank: Rank<'dt>,
}

#[derive(Deserialize, Debug)]
pub struct GetTopTracks<'dt> {
    #[serde(borrow)]
    pub track: Option<Vec<Track6<'dt>>>,
}

lastfm_t!(
    toptracks,
    GetTopTracks,
    _GetTopTracks,
    Method,
    UserGetTopTracks,
    Params,
    GetTopTracks,
    [
        user: &'rq str,
        period: Option<Period>,
        limit: Option<u32>,
        page: Option<u32>
    ]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Album3<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Option<Url<'dt>>,
    #[serde(deserialize_with="str_to_option")]
    pub playcount: Option<u32>,
    pub artist: Id1<'dt>,
    #[serde(rename="@attr")]
    pub rank: Option<Rank<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct GetWeeklyAlbumChart<'dt> {
    #[serde(borrow)]
    pub album: Option<Vec<Album3<'dt>>>,
}

lastfm_t!(
    weeklyalbumchart,
    GetWeeklyAlbumChart,
    _GetWeeklyAlbumChart,
    Method,
    UserGetWeeklyAlbumChart,
    Params,
    GetWeeklyAlbumChart,
    [
        user: &'rq str,
        from: Option<ChartDate>,
        to: Option<ChartDate>
    ]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Artist3<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Option<Url<'dt>>,
    #[serde(deserialize_with="str_to_val")]
    pub playcount: u32,
    #[serde(rename="@attr")]
    pub rank: Option<Rank<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct GetWeeklyArtistChart<'dt> {
    #[serde(borrow)]
    pub artist: Option<Vec<Artist3<'dt>>>,
}

lastfm_t!(
    weeklyartistchart,
    GetWeeklyArtistChart,
    _GetWeeklyArtistChart,
    Method,
    UserGetWeeklyArtistChart,
    Params,
    GetWeeklyArtistChart,
    [
        user: &'rq str,
        from: Option<ChartDate>,
        to: Option<ChartDate>
    ]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Track7<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    pub url: Option<Url<'dt>>,
    #[serde(deserialize_with="str_to_val")]
    pub playcount: u32,
    pub artist: Id1<'dt>,
    #[serde(rename="@attr")]
    pub rank: Option<Rank<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct GetWeeklyTrackChart<'dt> {
    #[serde(borrow)]
    pub track: Option<Vec<Track7<'dt>>>,
}

lastfm_t!(
    weeklytrackchart,
    GetWeeklyTrackChart,
    _GetWeeklyTrackChart,
    Method,
    UserGetWeeklyTrackChart,
    Params,
    GetWeeklyTrackChart,
    [
        user: &'rq str,
        from: Option<ChartDate>,
        to: Option<ChartDate>
    ]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Chart<'dt> {
    #[serde(rename="#text")]
    pub name: Option<&'dt str>,
    pub from: &'dt str,
    pub to: &'dt str,
}

#[derive(Deserialize, Debug)]
pub struct GetWeeklyChartList<'dt> {
    #[serde(borrow)]
    pub chart: Option<Vec<Chart<'dt>>>,
}

lastfm_t!(
    weeklychartlist,
    GetWeeklyChartList,
    _GetWeeklyChartList,
    Method,
    UserGetWeeklyChartList,
    Params,
    GetWeeklyChartList,
    [user: &'rq str]
);

// ----------------------------------------------------------------

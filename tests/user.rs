#![feature(decl_macro)]

extern crate lastfm_parse_rs as lastfm;

mod common;
use common::test_fn;
use lastfm::structs::user::{TaggingType, Period};


use lastfm::structs::user::GetArtistTracks;
test_fn!(
    test_user_getartisttracks,
    GetArtistTracks,
    ["xenzh", "the haunted", None, None, None]
);

use lastfm::structs::user::GetFriends;
test_fn!(
    test_user_getfriends,
    GetFriends,
    ["xenzh", Some(true), Some(3), None]
);

use lastfm::structs::user::GetInfo;
test_fn!(test_user_getinfo, GetInfo, ["xenzh"]);

use lastfm::structs::user::GetLovedTracks;
test_fn!(test_user_getlovedtracks, GetLovedTracks, ["xenzh", Some(5), None]);

use lastfm::structs::user::GetTaggings;
test_fn!(
    test_user_getperonaltags_artist,
    GetTaggings,
    ["xenzh", "folk metal", TaggingType::Artist, Some(3), None]
);
test_fn!(
    test_user_getperonaltags_track,
    GetTaggings,
    ["xenzh", "folk metal", TaggingType::Track, None, None]
);
test_fn!(
    test_user_getperonaltags_album,
    GetTaggings,
    ["xenzh", "folk metal", TaggingType::Album, Some(3), None]
);

use lastfm::structs::user::GetRecentTracks;
test_fn!(
    test_user_getrecenttracks,
    GetRecentTracks,
    ["xenzh", Some(true), None, None, Some(2), None]
);

use lastfm::structs::user::GetTopAlbums;
test_fn!(
    test_user_gettopalbums,
    GetTopAlbums,
    ["xenzh", Some(Period::OneYear), Some(6), None]
);

use lastfm::structs::user::GetTopArtists;
test_fn!(
    test_user_gettopartists,
    GetTopArtists,
    ["xenzh", Some(Period::OneYear), Some(4), Some(1)]
);

use lastfm::structs::user::GetTopTags;
test_fn!(
    test_user_gettoptags,
    GetTopTags,
    ["xenzh", Some(5)]
);

use lastfm::structs::user::GetTopTracks;
test_fn!(
    test_user_gettoptracks,
    GetTopTracks,
    ["xenzh", Some(Period::OneYear), Some(4), None]
);

use lastfm::structs::user::GetWeeklyAlbumChart;
test_fn!(
    test_user_getweeklyalbumchart,
    GetWeeklyAlbumChart,
    ["xenzh", None, None]
);

use lastfm::structs::user::GetWeeklyArtistChart;
test_fn!(
    test_user_getweeklyartistchart,
    GetWeeklyArtistChart,
    ["xenzh", None, None]
);

use lastfm::structs::user::GetWeeklyTrackChart;
test_fn!(
    test_user_getweeklytrackchart,
    GetWeeklyTrackChart,
    ["xenzh", None, None]
);

use lastfm::structs::user::GetWeeklyChartList;
test_fn!(
    test_user_getweeklychartlist,
    GetWeeklyChartList,
    ["xenzh"]
);

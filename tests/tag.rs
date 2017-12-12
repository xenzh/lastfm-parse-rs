#![feature(decl_macro)]
#![feature(try_from)]

extern crate lastfm_parse_rs as lastfm;

mod common;
use common::test_fn;

use lastfm::structs::tag::GetInfo;
test_fn!(test_tag_getinfo, GetInfo, ["ethno"]);

//17.11.2017 -- always returns an empty list
use lastfm::tag::GetSimilar;
test_fn!(test_tag_getsimilar, GetSimilar, ["low rock"]);

use lastfm::tag::GetTopAlbums;
test_fn!(test_tag_gettopalbums, GetTopAlbums, ["chamber pop", Some(2), None]);

use lastfm::tag::GetTopArtists;
test_fn!(test_tag_gettopartists, GetTopArtists, ["death metal", Some(2), Some(3)]);

use lastfm::tag::GetTopTags;
test_fn!(test_tag_gettoptags, GetTopTags, []);

use lastfm::tag::GetTopTracks;
test_fn!(test_tag_gettoptracks, GetTopTracks, ["cardiowave", Some(2), None]);

use lastfm::tag::GetWeeklyChartList;
test_fn!(test_tag_getweeklychartlist, GetWeeklyChartList, ["dark folk"]);
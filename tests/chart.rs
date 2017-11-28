#![feature(decl_macro)]

extern crate lastfm_parse_rs as lastfm;

mod common;
use common::test_fn;


use lastfm::structs::chart::GetTopArtists;
test_fn!(test_chart_gettopartists, GetTopArtists, [Some(2), None]);

use lastfm::structs::chart::GetTopTags;
test_fn!(test_chart_gettoptags, GetTopTags, [Some(6), None]);

use lastfm::structs::chart::GetTopTracks;
test_fn!(test_chart_gettoptracks, GetTopTracks, [Some(3), None]);

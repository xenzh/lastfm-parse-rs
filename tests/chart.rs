#![feature(decl_macro)]
#![feature(try_from)]

extern crate lastfm_parse_rs as lastfm;

mod common;
use common::test_fn;


use lastfm::chart::GetTopArtists;
test_fn!(test_chart_gettopartists, GetTopArtists, [Some(2), None]);

use lastfm::chart::GetTopTags;
test_fn!(test_chart_gettoptags, GetTopTags, [Some(6), None]);

use lastfm::chart::GetTopTracks;
test_fn!(test_chart_gettoptracks, GetTopTracks, [Some(3), None]);

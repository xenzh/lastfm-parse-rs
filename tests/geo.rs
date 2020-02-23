#![feature(decl_macro)]

extern crate lastfm_parse_rs as lastfm;

mod common;
use common::test_fn;

use lastfm::geo::GetTopArtists;
test_fn!(
    test_geo_gettopartists,
    GetTopArtists,
    ["ukraine", Some(3), Some(2)]
);

use lastfm::geo::GetTopTracks;
test_fn!(
    test_geo_gettoptracks,
    GetTopTracks,
    ["ukraine", None, Some(3),Some(5)]
);

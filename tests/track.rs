#![feature(decl_macro)]
#![feature(try_from)]

extern crate lastfm_parse_rs as lastfm;

mod common;
use common::test_fn;

use lastfm::track::GetCorrections;
test_fn!(test_track_getcorrection, GetCorrections, ["iamthemorning", "monsters"]);

use lastfm::track::GetInfo;
test_fn!(
    test_track_getinfo,
    GetInfo,
    ["nightwish", "come cover me", None, Some(1), Some("xenzh")]
);

use lastfm::track::GetSimilar;
test_fn!(
    test_track_getsimilar,
    GetSimilar,
    ["rainbow", "man on the silver mountain", None, Some(1), Some(3)]
);

use lastfm::track::GetTags;
test_fn!(
    test_track_gettags,
    GetTags,
    ["the finntronaut", "pronoun wars sjw death metal", None, Some(1), Some("xenzh")]
);

use lastfm::track::GetTopTags;
test_fn!(
    test_track_gettoptags,
    GetTopTags,
    ["cure", "creep", None, Some(1)]
);

use lastfm::track::Search;
test_fn!(
    test_track_search,
    Search,
    ["scorpions", "change", Some(3), Some(1)]
);
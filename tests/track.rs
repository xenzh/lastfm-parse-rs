#![feature(decl_macro)]

extern crate lastfm_parse_rs as lastfm;

mod common;
use common::test_fn;

use lastfm::structs::track::Corrections;
test_fn!(test_track_getcorrection, Corrections, ["iamthemorning", "monsters"]);

use lastfm::structs::track::Info;
test_fn!(
    test_track_getinfo,
    Info,
    ["nightwish", "come cover me", None, Some(1), Some("xenzh")]
);

use lastfm::structs::track::SimilarList;
test_fn!(
    test_track_getsimilar,
    SimilarList,
    ["rainbow", "man on the silver mountain", None, Some(1), Some(3)]
);

use lastfm::structs::track::UserTags;
test_fn!(
    test_track_gettags,
    UserTags,
    ["the finntronaut", "pronoun wars sjw death metal", None, Some(1), Some("xenzh")]
);

use lastfm::structs::track::TopTags;
test_fn!(
    test_track_gettoptags,
    TopTags,
    ["cure", "creep", None, Some(1)]
);

use lastfm::structs::track::Search;
test_fn!(
    test_track_search,
    Search,
    ["scorpions", "change", Some(3), Some(1)]
);
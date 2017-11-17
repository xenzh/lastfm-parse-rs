#![feature(decl_macro)]

extern crate lastfm_parse_rs as lastfm;

mod common;
use common::test_fn;

use lastfm::structs::artist::Corrections;
test_fn!(test_artist_getcorrection, Corrections, ["guns roses"]);

use lastfm::structs::artist::Info;
test_fn!(
    test_artist_getinfo,
    Info,
    ["iamthemorning", None, Some(1), None, Some("xenzh")]
);

use lastfm::structs::artist::SimilarList;
test_fn!(test_artist_getsimilar, SimilarList, ["nadja", None, Some(1), Some(3)]);

use lastfm::structs::artist::UserTags;
test_fn!(
    test_artist_gettags,
    UserTags,
    ["adam lane's full throttle orchestra", None, None, Some("xenzh")]
);

use lastfm::structs::artist::TopAlbums;
test_fn!(
    test_artist_gettopalbums,
    TopAlbums,
    ["days n' daze", None, Some(1), Some(4), None]
);

use lastfm::structs::artist::TopTags;
test_fn!(test_artist_gettoptags, TopTags, ["schtimm", None, Some(1)]);

use lastfm::structs::artist::TopTracks;
test_fn!(
    test_artist_gettoptracks,
    TopTracks,
    ["charles mingus", None, Some(1), Some(4), None]
);

use lastfm::structs::artist::Search;
test_fn!(test_artist_search, Search, ["days", Some(3), Some(2)]);
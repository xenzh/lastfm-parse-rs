#![feature(decl_macro)]

extern crate lastfm_parse_rs as lastfm;

mod common;
use common::test_fn;

use lastfm::structs::album::GetInfo;
test_fn!(
    test_album_getinfo,
    GetInfo,
    ["hannah fury", "subterfuge", None, Some(1), None, Some("xenzh")]
);

use lastfm::structs::album::GetUserTags;
test_fn!(
    test_album_gettags,
    GetUserTags,
    ["rome", "the hyperion machine", None, Some(1), Some("xenzh")]
);

use lastfm::structs::album::GetTopTags;
test_fn!(
    test_album_gettoptags,
    GetTopTags,
    ["the bad plus", "never stop", None, Some(1)]
);

use lastfm::structs::album::Search;
test_fn!(test_album_search, Search, ["clarity", Some(4), None]);
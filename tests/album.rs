#![feature(decl_macro)]

extern crate lastfm_parse_rs as lastfm;

mod common;
use common::test_fn;

use lastfm::structs::album::Info;
test_fn!(
    test_album_getinfo,
    Info,
    ["hannah fury", "subterfuge", None, Some(1), None, Some("xenzh")]
);

use lastfm::structs::album::UserTags;
test_fn!(
    test_album_gettags,
    UserTags,
    ["rome", "the hyperion machine", None, Some(1), Some("xenzh")]
);

use lastfm::structs::album::TopTags;
test_fn!(
    test_album_gettoptags,
    TopTags,
    ["the bad plus", "never stop", None, Some(1)]
);

use lastfm::structs::album::Search;
test_fn!(test_album_search, Search, ["clarity", Some(4), None]);
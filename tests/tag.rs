#![feature(decl_macro)]

extern crate lastfm_parse_rs;

mod common;
use common::test_fn;

use lastfm_parse_rs::structs::tag::Info as TagInfo;
test_fn!(auto_new_tag_test, TagInfo, ["ethno"]);

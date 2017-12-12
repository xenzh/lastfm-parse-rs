#![feature(decl_macro)]
#![feature(try_from)]

extern crate lastfm_parse_rs as lastfm;

mod common;
use common::test_fn;

use lastfm::library::GetArtists;
test_fn!(test_library_getartists, GetArtists, ["xenzh", Some(3), None]);
#![feature(decl_macro)]

extern crate lastfm_parse_rs as lastfm;

mod common;
use common::test_fn;

use lastfm::structs::tag::Info;
test_fn!(test_tag_getinfo, Info, ["ethno"]);

//17.11.2017 -- always returns an empty list
use lastfm::structs::tag::SimilarList;
test_fn!(test_tag_getsimilar, SimilarList, ["low rock"]);

use lastfm::structs::tag::TopAlbums;
test_fn!(test_tag_gettopalbums, TopAlbums, ["chamber pop", Some(2), None]);

use lastfm::structs::tag::TopArtists;
test_fn!(test_tag_gettopartists, TopArtists, ["death metal", Some(2), Some(3)]);

use lastfm::structs::tag::TopTags;
test_fn!(test_tag_gettoptags, TopTags, []);

use lastfm::structs::tag::TopTracks;
test_fn!(test_tag_gettoptracks, TopTracks, ["cardiowave", Some(2), None]);

use lastfm::structs::tag::WeeklyChartList;
test_fn!(test_tag_getweeklychartlist, WeeklyChartList, ["dark folk"]);
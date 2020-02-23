#![feature(decl_macro)]

extern crate lastfm_parse_rs as lastfm;

use lastfm::from_json_str;

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

use lastfm::track::Scrobble;

#[test]
fn test_track_scrobble_single() {
    let raw_json = r##"
{
  "scrobbles": {
    "@attr": {
      "accepted": 1,
      "ignored": 0
    },
    "scrobble": {
      "artist": {
        "corrected": "0",
        "#text": "iamthemorning"
      },
      "ignoredMessage": {
        "code": "0",
        "#text": ""
      },
      "albumArtist": {
        "corrected": "0",
        "#text": ""
      },
      "timestamp": "1513154253",
      "album": {
        "corrected": "0"
      },
      "track": {
        "corrected": "0",
        "#text": "touching ii"
      }
    }
  }
}
    "##;

    let data: Scrobble = from_json_str(&raw_json).unwrap();
    println!("\nDeserialized {}:\n{:?}", stringify!(Scrobble), data);
}

#[test]
fn test_track_scrobble_batch() {
    let raw_json = r##"
{
  "scrobbles": {
    "@attr": {
      "accepted": 2,
      "ignored": 0
    },
    "scrobble": [
      {
        "album": {
          "corrected": "0"
        },
        "albumArtist": {
          "#text": "",
          "corrected": "0"
        },
        "artist": {
          "#text": "iamthemorning",
          "corrected": "0"
        },
        "ignoredMessage": {
          "#text": "",
          "code": "0"
        },
        "timestamp": "1513154280",
        "track": {
          "#text": "touching ii",
          "corrected": "0"
        }
      },
      {
        "album": {
          "corrected": "0"
        },
        "albumArtist": {
          "#text": "",
          "corrected": "0"
        },
        "artist": {
          "#text": "schtimm",
          "corrected": "0"
        },
        "ignoredMessage": {
          "#text": "",
          "code": "0"
        },
        "timestamp": "1513154299",
        "track": {
          "#text": "sunotic drive",
          "corrected": "0"
        }
      }
    ]
  }
}
    "##;

    let data: Scrobble = from_json_str(&raw_json).unwrap();
    println!("\nDeserialized {}:\n{:?}", stringify!(Scrobble), data);
}

//! Collection of serde-based data types for Lastfm API
//!
//! Source: https://www.last.fm/api
//!
//! ## Installation
//!
//! ## Example
//!
//! ```no-run
//! use lastfm_parse_rs::from_json_str;
//! use lastfm_parse_rs::structs::artist::GetInfo;
//!
//! let get_info = GetInfo::request(base_url, api_key, "iamthemorning", None, Some(1), None, None);
//! let url: Url = Into::into(get_info);
//! // Make an http request
//! let data: GetInfo = from_json_str(&raw_json_response).unwrap();
//! ```
//!
//! See examples/artist.rs for full listing.
//!
//! ## Tests
//!
//! To run deserialization tests:
//!
//! 1. Set your lastfm API key to \tests\common\mod.rs `LASTFM_API_KEY`.
//!
//! 2. Run `cargo test`. Use  `-- --nocapture` to dump raw json and deserialized objects to stdout.
//!
//! To run examples:
//!
//! 1. Set your lastfm API key to examples/*.rs
//!
//! 2. Run `cargo run --example <example name>`

#![feature(try_from)]

extern crate serde;
extern crate serde_json;
extern crate url;
extern crate md5;

#[macro_use]
extern crate serde_derive;

// ----------------------------------------------------------------

/// Tools for defining common Lastfm data structures and corresponding requests
#[macro_use]
pub mod lastfm_type;
/// Tools for constructing API requests
#[macro_use]
pub mod request;

/// Serde-based API data structures
pub mod structs;
/// Common error type for serde/API fails
pub mod error;

// ----------------------------------------------------------------

pub use lastfm_type::{LastfmType, from_json_str, from_json_slice};
pub use request::{Request, RequestParams};
pub use error::Result;

// ----------------------------------------------------------------

/// Album data structures
pub mod album {
    pub use structs::album::Params;

    pub use structs::album::GetInfo;
    pub use structs::album::GetTags;
    pub use structs::album::GetTopTags;
    pub use structs::album::Search;
}

/// Artist data structures
pub mod artist {
    pub use structs::artist::Params;

    pub use structs::artist::GetCorrections;
    pub use structs::artist::GetInfo;
    pub use structs::artist::GetSimilar;
    pub use structs::artist::GetTags;
    pub use structs::artist::GetTopAlbums;
    pub use structs::artist::GetTopTags;
    pub use structs::artist::GetTopTracks;
    pub use structs::artist::Search;
}

/// Auth data structures
pub mod auth {
    pub use structs::auth::Params;

    pub use structs::auth::GetMobileSession;
    pub use structs::auth::GetSession;
    pub use structs::auth::GetToken;
}

/// Chart data structures
pub mod chart {
    pub use structs::chart::Params;

    pub use structs::chart::GetTopArtists;
    pub use structs::chart::GetTopTags;
    pub use structs::chart::GetTopTracks;
}
/// Geo data structures
pub mod geo {
    pub use structs::geo::Params;

    pub use structs::geo::GetTopArtists;
    pub use structs::geo::GetTopTracks;
}

/// Library data structures
pub mod library {
    pub use structs::library::Params;

    pub use structs::library::GetArtists;
}
/// Tag data structures
pub mod tag {
    pub use structs::tag::Params;

    pub use structs::tag::GetInfo;
    pub use structs::tag::GetSimilar;
    pub use structs::tag::GetTopAlbums;
    pub use structs::tag::GetTopArtists;
    pub use structs::tag::GetTopTags;
    pub use structs::tag::GetTopTracks;
    pub use structs::tag::GetWeeklyChartList;
}
/// Track data structures
pub mod track {
    pub use structs::track::Params;

    pub use structs::track::GetCorrections;
    pub use structs::track::GetInfo;
    pub use structs::track::GetSimilar;
    pub use structs::track::GetTags;
    pub use structs::track::GetTopTags;
    pub use structs::track::Search;
}
/// User data structures
pub mod user {
    pub use structs::user::Params;

    pub use structs::user::GetArtistTracks;
    pub use structs::user::GetFriends;
    pub use structs::user::GetInfo;
    pub use structs::user::GetLovedTracks;
    pub use structs::user::GetTaggings;
    pub use structs::user::GetRecentTracks;
    pub use structs::user::GetTopAlbums;
    pub use structs::user::GetTopArtists;
    pub use structs::user::GetTopTags;
    pub use structs::user::GetTopTracks;
    pub use structs::user::GetWeeklyAlbumChart;
    pub use structs::user::GetWeeklyArtistChart;
    pub use structs::user::GetWeeklyTrackChart;
    pub use structs::user::GetWeeklyChartList;
}

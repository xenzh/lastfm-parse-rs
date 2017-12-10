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

extern crate serde;
extern crate serde_json;
extern crate url;

#[macro_use]
extern crate serde_derive;

// ----------------------------------------------------------------

/// Tools for defining common Lastfm data structures and corresponding requests
#[macro_use]
pub mod lastfm_type;
/// Tools for constructing API requests
pub mod request;

/// Serde-based API data structures
pub mod structs;
/// Common error type for serde/API fails
pub mod error;

/// Album data structures
pub mod album;
/// Artist data structures
pub mod artist;
/// Chart data structures
pub mod chart;
/// Geo data structures
pub mod geo;
/// Library data structures
pub mod library;
/// Tag data structures
pub mod tag;
/// Track data structures
pub mod track;
/// User data structures
pub mod user;

// ----------------------------------------------------------------

pub use lastfm_type::{LastfmType, from_json_str, from_json_slice};
pub use request::{Request, RequestParams};
pub use error::Result;

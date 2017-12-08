extern crate serde;
extern crate serde_json;
extern crate url;

#[macro_use]
extern crate serde_derive;

// ----------------------------------------------------------------

#[macro_use]
pub mod lastfm_type;

pub mod structs;
pub mod error;
pub mod methods;

// ----------------------------------------------------------------

pub use lastfm_type::{LastfmType, from_json_str, from_json_slice};
pub use error::Result;

// ----------------------------------------------------------------

// To run deserialization tests:
// 1. Set your lastfm API key in \tests\common\mod.rs LASTFM_API_KEY.
// 2. Run 'cargo test -- --nocapture' to dump raw json and deserialized objects to stdout.

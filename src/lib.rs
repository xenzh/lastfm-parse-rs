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

/// Deserializetion tests
/// They query lastfm's REST API and parse json responses into objects.
/// Run 'cargo test -- --nocapture' to dump deserialized objects to stdout.
#[cfg(test)]
mod tests;

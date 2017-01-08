#![feature(proc_macro)]

extern crate serde;
extern crate serde_json;
extern crate url;

#[macro_use]
extern crate serde_derive;

pub mod common;
pub mod album;
pub mod artist;
pub mod tag;
pub mod track;
pub mod user;

pub mod api_error;

// TODO: add proper composamble error handling for ApiError and serde_json::Error
#[macro_export]
macro_rules! lastfm_obj_from_json {
    ($ctype:ty, $attrname:ident, $json:expr) => {
        {
            #[derive(Deserialize, Debug)]
            pub struct ObjWrapper {
                pub $attrname: $ctype,
            }
            let obj: ObjWrapper = serde_json::from_str(&$json).unwrap();
            obj.$attrname
        }
    };
}

/// Deserializetion tests
/// They query lastfm's REST API and parse json responses into objects.
/// Run 'cargo test -- --nocapture' to dump deserialized objects to stdout.
#[cfg(test)]
mod tests;
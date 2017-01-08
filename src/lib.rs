#![feature(proc_macro)]

extern crate serde;
extern crate url;

#[macro_use]
extern crate serde_derive;

// ----------------------------------------------------------------

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

// ----------------------------------------------------------------

mod common;
mod album;
mod artist;
mod tag;
mod track;
mod user;


/// Deserializetion tests
/// They query lastfm's REST API and parse json responses into objects.
/// Run 'cargo test -- --nocapture' to dump deserialized objects to stdout.
#[cfg(test)]
mod tests;

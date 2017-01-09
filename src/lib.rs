#![feature(proc_macro)]

extern crate serde;
extern crate serde_json;
extern crate url;

#[macro_use]
extern crate serde_derive;

// ----------------------------------------------------------------

// Generates a wrapper over root json object
// Following should be included in order to use this macros:
// use std::convert::Into;
// use common::Wrapped;
macro_rules! wrapper_t {
    ($name:ident, $wrapped:ident, $wrapped_t:ty) => {
        #[derive(Deserialize, Debug)]
        pub struct $name {
            $wrapped: $wrapped_t,
        }
        impl Into<$wrapped_t> for $name {
            fn into(self) -> $wrapped_t {
                self.$wrapped
            }
        }
        impl Wrapped for $wrapped_t {
            type Outer = $name;
        }
    }
}

// ----------------------------------------------------------------

pub mod common;
pub mod album;
pub mod artist;
pub mod tag;
pub mod track;
pub mod user;

pub mod api_error;

/// Deserializetion tests
/// They query lastfm's REST API and parse json responses into objects.
/// Run 'cargo test -- --nocapture' to dump deserialized objects to stdout.
#[cfg(test)]
mod tests;

// ----------------------------------------------------------------

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::error::Error as StdError;
use std::result::Result as StdResult;

use api_error::ApiError;
use serde_json::error::Error as SerdeError;

use serde::de::Deserialize;


#[derive(Debug)]
pub enum Error {
    Deserialize(SerdeError),
    Api(ApiError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Error::Deserialize(ref se) => {
                write!(f, "Deserialization failed, reason: {}", se.description())
            }
            Error::Api(ref ae) => {
                write!(f, "Lastfm API error\n")?;
                ae.fmt(f)
            }
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Deserialize(ref se) => se.description(),
            Error::Api(ref ae) => ae.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Deserialize(ref se) => Some(se),
            _ => None,
        }
    }
}

pub type Result<T> = StdResult<T, Error>;

// ----------------------------------------------------------------

use std::convert::Into;
use common::Wrapped;

pub fn lastfm_obj_from_json<T>(json: &str) -> Result<T>
    where T: Deserialize + Wrapped
{
    let res: Result<T::Outer> = serde_json::from_str(&json).map_err(|e| Error::Deserialize(e));
    if res.is_err() {
        let err_res: Result<ApiError> = serde_json::from_str(&json)
            .map_err(|e| Error::Deserialize(e));

        if err_res.is_ok() {
            return Err(Error::Api(err_res.unwrap()));
        }
    }
    Ok(res.unwrap().into())
}

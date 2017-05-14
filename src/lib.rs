#![feature(proc_macro)]

extern crate serde;
extern crate serde_json;
extern crate url;

#[macro_use]
extern crate serde_derive;

// ----------------------------------------------------------------

#[macro_use]
pub mod common;
pub mod album;
pub mod artist;
pub mod tag;
pub mod track;
pub mod chart;
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
use std::convert::Into;

use serde_json::error::Error as SerdeError;
use serde::de::Deserialize;

use api_error::ApiError;
use common::Wrapped;

// ----------------------------------------------------------------

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

pub fn lastfm_obj_from_json<'de, T>(json: &'de str) -> Result<T>
    where T: Deserialize<'de> + Wrapped<'de>
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

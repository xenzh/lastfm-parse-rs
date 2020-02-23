use std::fmt::{Display, Formatter, Result as FmtResult};
use std::error::Error as StdError;
use std::result::Result as StdResult;

use serde_json::error::Error as SerdeError;

pub use structs::api_error::ApiError;

/// Error type for Last.fm API requests
/// As of now, it can be either serde or API fail.
/// Serde fails usually mean that the problem is in the library.
#[derive(Debug)]
pub enum Error {
    Deserialize(SerdeError),
    Api(ApiError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Error::Deserialize(ref se) => {
                write!(f, "Deserialization failed, reason: {}", se.to_string())
            }
            Error::Api(ref ae) => {
                write!(f, "Lastfm API error\n")?;
                ae.fmt(f)
            }
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Error::Deserialize(ref se) => Some(se),
            _ => None,
        }
    }
}

pub type Result<T> = StdResult<T, Error>;

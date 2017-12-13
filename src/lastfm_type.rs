use std::marker::Sized;
use std::convert::Into;
use std::fmt::Debug;

use serde::de::Deserialize;
use serde_json;

use error::{Error, Result};
use structs::api_error::ApiError;

pub use request::{Request, RequestParams};

// ----------------------------------------------------------------

/// All lastfm json data types are wrapped with an extra object. Like this:
/// `{ tag: { /* actual fields */ } }`
/// While Rust object structure has to match json's in order for serde to work,
/// this extra wrapping is not very convenient from users perspective.
/// This trait indicates that type is wrapped and provides deserializable
/// wrapper struct type along with inner type conversion.
/// This trait is used internally to automatically generate wrapper objects and
/// hide them from user (see lastfm_t! macro).
pub trait LastfmType<'de>
where
    Self: Sized + Deserialize<'de>,
{
    type Outer: Deserialize<'de> + Debug + Into<Self>;
}

// ----------------------------------------------------------------

macro_rules! from_json_impl {
    ($defn:path, $src:expr, $dst:ty) => {{
        let res: Result<$dst> = $defn($src).map_err(|e| Error::Deserialize(e));
        if res.is_err() {
            let err_res: Result<ApiError> =
                $defn($src).map_err(|e| Error::Deserialize(e));

            return match err_res {
                Ok(api_err) => Err(Error::Api(api_err)),
                Err(_) => Err(res.unwrap_err()),
            };
        }
        Ok(res.unwrap().into())
    }}
}

/// Parses given data type from json string slice (zero-copy)
/// Beware that it will fail in case source contains escape sequences,
/// as serde is currently unable to decode them inplace.
/// Current solution is to unescape input manually beforehand.
/// See https://github.com/serde-rs/json/issues/318 and tests/common/mod.rs for more details.
pub fn from_json_str<'de, Lt: LastfmType<'de>>(json: &'de str) -> Result<Lt> {
    from_json_impl!(serde_json::from_str, &json, Lt::Outer)
}

/// Parses given data type from json byte slice (zero-copy)
/// Beware that it will fail in case source contains escape sequences,
/// as serde is currently unable to decode them inplace.
/// Current solution is to unescape input manually beforehand.
/// See https://github.com/serde-rs/json/issues/318 and tests/common/mod.rs for more details.
pub fn from_json_slice<'de, Lt: LastfmType<'de>>(json: &'de [u8]) -> Result<Lt> {
    from_json_impl!(serde_json::from_slice, &json, Lt::Outer)
}

// ----------------------------------------------------------------

/// This macro is used to define top-level requestable Lastfm data structure.
/// For given Deserialize + Debug struct specifies
/// a wrapper (see LastfmType trait), conversions
/// and request() function with API method agruments.
/// Following should be included in order to use this macro:
/// ```
/// use std::convert::Into;
/// use lastfm_type::{LastfmType, Request};
/// ```
#[macro_export]
macro_rules! lastfm_t {
    (
        $name:ident, $data_t:ident, $wrapper_name:ident,
        $params_t:ident, $params_variant:ident,
        [$($param_key:ident: $param_t:ty),*]
    ) => {
        #[derive(Deserialize, Debug)]
        pub struct $wrapper_name<'dt> {
            #[serde(borrow)]
            $name: $data_t<'dt>,
        }

        impl<'dt> Into<$data_t<'dt>> for $wrapper_name<'dt> {
            fn into(self) -> $data_t<'dt> {
                self.$name
            }
        }

        impl<'dt> LastfmType<'dt> for $data_t<'dt> {
            type Outer = $wrapper_name<'dt>;
        }

        impl<'dt> $data_t<'dt> {
            pub fn request<'rq>(
                base_url: &'rq str,
                api_key: &'rq str,
                secret: Option<&'rq str>,
                session: Option<&'rq str>,
                $($param_key: $param_t,)*
            ) -> Request<'rq, $params_t<'rq>> {
                Request {
                    base_url: base_url,
                    api_key: api_key,
                    secret: secret,
                    session: session,
                    params: $params_t::$params_variant { $($param_key: $param_key,)* },
                }
            }
        }
    }
}

#[macro_export]
macro_rules! unwrapped_lastfm_t {
    (
        $data_t:ident, $params_t:ident, $params_variant:ident,
        [$($param_key:ident: $param_t:ty),*]
    ) => {
        impl<'dt> LastfmType<'dt> for $data_t<'dt> {
            type Outer = Self;
        }

        impl<'dt> $data_t<'dt> {
            pub fn request<'rq>(
                base_url: &'rq str,
                api_key: &'rq str,
                secret: Option<&'rq str>,
                session: Option<&'rq str>,
                $($param_key: $param_t,)*
            ) -> Request<'rq, $params_t<'rq>> {
                Request {
                    base_url: base_url,
                    api_key: api_key,
                    secret: secret,
                    session: session,
                    params: $params_t::$params_variant { $($param_key: $param_key,)* },
                }
            }
        }
    }
}

// ----------------------------------------------------------------

/// Generates lastfm_t wrapper over an opensearch object
/// Following should be included in order to use this macro:
/// ```
/// use common::SearchQuery;
/// ```
#[macro_export]
macro_rules! opensearch_t {
    (
        $name:ident, $data_name:ident, $wrapper_name:ident,
        $result:ident, $result_t:ident,
        $params_t:ident, $params_variant:ident,
        [$($param_key:ident: $param_t:ty),*]
    ) => {
        #[derive(Deserialize, Debug)]
        pub struct $data_name<'dt> {
            #[serde(rename="opensearch:Query")]
            #[serde(borrow)]
            pub query: SearchQuery<'dt>,
            #[serde(rename="opensearch:totalResults")]
            #[serde(deserialize_with="str_to_option")]
            pub total_results: Option<u32>,
            #[serde(rename="opensearch:startIndex")]
            #[serde(deserialize_with="str_to_option")]
            pub start_index: Option<u32>,
            #[serde(rename="opensearch:itemsPerPage")]
            #[serde(deserialize_with="str_to_option")]
            pub iterms_per_page: Option<u32>,
            #[serde(borrow)]
            pub $result: Option<$result_t<'dt>>,
        }

        lastfm_t!(
            $name,
            $data_name,
            $wrapper_name,
            $params_t,
            $params_variant,
            [ $($param_key: $param_t),* ]
        );
    }
}

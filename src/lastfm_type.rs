use std::marker::Sized;
use std::convert::Into;
use std::fmt::Debug;

use url::Url;
use serde::de::Deserialize;
use serde_json;

use error::{Error, Result};
use structs::api_error::ApiError;
use methods::Method;

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

// Note: this can't be an associated method b/c result's lifetime should be taken
// from json string, not the type it's called from.
// Or, rather, it can be done but will look really awkward.
pub fn from_json<'de, Lt: LastfmType<'de>>(json: &'de str) -> Result<Lt> {
    let res: Result<Lt::Outer> = serde_json::from_str(&json).map_err(|e| Error::Deserialize(e));
    if res.is_err() {
        let err_res: Result<ApiError> =
            serde_json::from_str(&json).map_err(|e| Error::Deserialize(e));

        return match err_res {
            Ok(api_err) => Err(Error::Api(api_err)),
            Err(_) => Err(res.unwrap_err()),
        };
    }
    Ok(res.unwrap().into())
}

/// Temporary solution: a trait for request parameter type that makes
/// this type to know how to add itself to an url.
/// Better solution would be to make every patameters type to implement IntoIterator
/// providing ability to iterate over request pairs.
pub trait RequestParams {
    fn append_to(&self, url: &mut Url);
}

/// Request information associated with a method and lastfm data type.
/// Can be converted to a Url
#[derive(Debug)]
pub struct Request<'rq, T>
where
    T: RequestParams + Debug,
{
    pub base_url: &'rq str,
    pub api_key: &'rq str,
    pub method: Method,
    pub params: T,
}

impl<'rq, T> Into<Url> for Request<'rq, T>
where
    T: RequestParams + Debug,
{
    fn into(self) -> Url {
        let mut url =
            Url::parse(self.base_url).expect("Base url is incorrect. How did this even happen?");
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("api_key", self.api_key);
            query_pairs.append_pair("format", "json");
            query_pairs.append_pair("method", self.method.api_name());
        }
        self.params.append_to(&mut url);
        url
    }
}

// ----------------------------------------------------------------

// For given Deserialize + Debug struct specifies:
//   * wrapper (see LastfmType trait) and conversions
//   * impl with fn request(&self) -> Request<RequestParameters>
// Following should be included in order to use this macro:
//   use std::convert::Into;
//   use utils::{LastfmType, Request};
#[macro_export]
macro_rules! lastfm_t {
    (
        $name:ident, $data_t:ident, $wrapper_name:ident,
        $method_t:ident, $method_variant:ident,
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
            pub fn request<'rq>(base_url: &'rq str, api_key: &'rq str, $($param_key: $param_t,)*) -> Request<'rq, $params_t<'rq>> {
                Request {
                    base_url: base_url,
                    api_key: api_key,
                    method: $method_t::$method_variant,
                    params: $params_t::$params_variant { $($param_key: $param_key,)* },
                }
            }
        }
    }
}

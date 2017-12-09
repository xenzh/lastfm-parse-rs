use std::fmt::Debug;

use url::Url;
use methods::Method;

// ----------------------------------------------------------------

/// A trait for request parameter type that makes this type appendable to an url.
pub trait RequestParams {
    fn append_to(&self, url: &mut Url);
}

/// Request information associated with a method and lastfm data type.
/// Can be converted to a Url. No POST request support yet.
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

impl<'rq, T> Request<'rq, T>
where
    T: RequestParams + Debug,
{
    /// Constructs new Request from given method and parameters
    pub fn new(base_url: &'rq str, api_key: &'rq str, method: Method, params: T) -> Request<'rq, T> {
        Request { base_url, api_key, method, params }
    }

    /// Converts Request object to an Url, appends request parameters (GET only)
    pub fn as_url(&self) -> Url {
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

impl<'rq, T> Into<Url> for Request<'rq, T>
where
    T: RequestParams + Debug,
{
    fn into(self) -> Url {
        (&self).as_url()
    }
}
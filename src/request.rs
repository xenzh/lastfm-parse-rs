use std::fmt::Debug;
use std::io::{Error, ErrorKind, Result};
use std::convert::TryFrom;

use url::Url;
use md5::compute as md5;

// ----------------------------------------------------------------

/// Describes API method requests
pub trait RequestParams {
    /// Returns method name
    fn method(&self) -> &str;
    /// If `true`, it means that this method is "write" (see https://www.last.fm/api/rest).
    /// It means it has to be signed and all parameters should be transferred via POST.
    fn is_write(&self) -> bool;
    /// Appends method parameters to given url.
    /// Please note that common and special parameters like method name, api key, session key and signature
    /// are appended automatically somewhere else
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
    pub secret: Option<&'rq str>,
    pub params: T,
}

impl<'rq, T> Request<'rq, T>
where
    T: RequestParams + Debug,
{
    /// Constructs new Request from given method and parameters
    pub fn new(base_url: &'rq str, api_key: &'rq str, secret: Option<&'rq str>, params: T) -> Request<'rq, T> {
        Request { base_url, api_key, secret, params }
    }

    /// Converts Request object to an Url.
    /// Automatically determines whether to sign it ("write" methods).
    pub fn get_url(&self) -> Result<Url> {
        if self.params.is_write() {
            self.get_write_url()
        } else {
            self.get_read_url()
        }
    }

    /// Converts Request object to an Url, appends request parameters ("read" methods, GET)
    pub fn get_read_url(&self) -> Result<Url> {
        let mut url = Url::parse(self.base_url).map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("api_key", self.api_key);
            query_pairs.append_pair("format", "json");
            query_pairs.append_pair("method", self.params.method());
        }
        self.params.append_to(&mut url);
        Ok(url)
    }

    /// Converts request object to a url, appends request parameters.
    /// Also "signs" the call by adding api_sig argument as described in
    /// https://www.last.fm/api/mobileauth#4 ("write" methods, POST)
    pub fn get_write_url(&self) -> Result<Url> {
        let mut base = self.get_read_url()?;

        let mut signature = {
            let mut pairs = base.query_pairs()
                .filter(|&(ref name, _)| name != "format" && name != "callback")
                .collect::<Vec<_>>();

            pairs.sort_unstable();

            pairs.into_iter().fold(String::new(), |mut acc, (ref n, ref v)| {
                acc.push_str(n);
                acc.push_str(v);
                acc
            })
        };
        signature.push_str(self.secret.ok_or(
            Error::new(ErrorKind::InvalidInput,
                "No secret specified but write url requested")
        )?);

        let digest = format!("{:x}", md5(signature));
        base.query_pairs_mut().append_pair("api_sig", &digest);
        Ok(base)
    }
}

impl<'rq, T> TryFrom<Request<'rq, T>> for Url
where
    T: RequestParams + Debug
{
    type Error = Error;

    fn try_from(rq: Request<'rq, T>) -> Result<Url> {
        rq.get_url()
    }
}

// ----------------------------------------------------------------

#[macro_export]
macro_rules! cv {
    ($what:ident) => { &$what.to_string() }
}

#[macro_export]
macro_rules! params {
    (
        $to:ident,
        [$($rqn:ident: $rqe:expr),*],
        [$($opn:ident: $ope:expr),*]
    ) => {
        $( $to.append_pair(stringify!($rqn), $rqe); )*
        $( if let Some($opn) = $opn { $to.append_pair(stringify!($opn), $ope); }; )*
    }
}

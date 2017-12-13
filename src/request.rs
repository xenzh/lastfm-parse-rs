use std::fmt::Debug;
use std::io::{Error, ErrorKind, Result};
use std::convert::TryFrom;

use url::Url;
use md5::compute as md5;

// ----------------------------------------------------------------

/// Describes API method requests
pub trait RequestParams {

    /// Returns API method name
    fn method(&self) -> &str;

    /// Appends method parameters to given url.
    /// Please note that common and special parameters like method name, api key, session key and signature
    /// are appended automatically somewhere else
    fn append_to(&self, url: &mut Url);

    /// Most `write` (see https://www.last.fm/api/rest) API methods (except auth) need to be signed.
    /// This function indicates whether given method is one of these.
    fn needs_signature(&self) -> bool;

    /// All `write` (see https://www.last.fm/api/rest) API methods require user to be authenticated.
    /// For them session key has to be additionally provided in request body.
    fn needs_session_key(&self) -> bool;
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
    pub session: Option<&'rq str>,
    pub params: T,
}

impl<'rq, T> Request<'rq, T>
where
    T: RequestParams + Debug,
{
    /// Constructs new Request from given method and parameters
    pub fn new(
        base_url: &'rq str,
        api_key: &'rq str,
        secret: Option<&'rq str>,
        session: Option<&'rq str>,
        params: T
    ) -> Request<'rq, T> {
        Request { base_url, api_key, secret, session, params }
    }

    /// Converts Request object to an Url and appends method parameters to the query.
    /// * Appends a session key for `write` API methods.
    /// * Signs `write` and `auth` API methods.
    pub fn get_url(&self) -> Result<Url> {
        let mut url = self.make_url()?;

        if self.params.needs_session_key() {
            url = self.append_session_key(url)?;
        }
        if self.params.needs_signature() {
            url = self.sign_url(url)?;
        }

        Ok(url)
    }

    fn make_url(&self) -> Result<Url> {
        let mut url = Url::parse(self.base_url)
            .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
        {
            let mut query = url.query_pairs_mut();
            query.append_pair("api_key", self.api_key);
            query.append_pair("format", "json");
            query.append_pair("method", self.params.method());
        }
        self.params.append_to(&mut url);
        Ok(url)
    }

    fn append_session_key(&self, mut base: Url) -> Result<Url> {
        let sk = self.session.ok_or(
            Error::new(ErrorKind::InvalidInput, "Missing session key"))?;
        {
            let mut query = base.query_pairs_mut();
            query.append_pair("sk", sk);
        }
        Ok(base)
    }

    fn sign_url(&self, mut base: Url) -> Result<Url> {
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
                "All methods that need signature also need secret")
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

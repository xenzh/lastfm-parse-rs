use url::Url as StdUrl;

use lastfm_type::{LastfmType, Request, RequestParams};

// ----------------------------------------------------------------

#[derive(Debug)]
pub enum Params<'pr> {
    GetMobileSession {
        username: &'pr str,
        password: &'pr str,
    },
    GetSession { token: &'pr str, },
    GetToken,
}

impl<'pr> RequestParams for Params<'pr> {
    fn method(&self) -> &str {
        match *self {
            Params::GetMobileSession { .. } => "auth.getmobilesession",
            Params::GetSession { .. } => "auth.getsession",
            Params::GetToken => "auth.gettoken",
        }
    }

    fn needs_signature(&self) -> bool {
        true
    }

    fn needs_session_key(&self) -> bool {
        false
    }

    fn append_to(&self, url: &mut StdUrl) {
        let mut query = url.query_pairs_mut();
        match *self {
            Params::GetMobileSession { username, password } => {
                params!(query, [username: username, password: password], []);
            }
            Params::GetSession { token } => {
                params!(query, [token: token], []);
            }
            Params::GetToken => {}
        }
    }
}

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct GetMobileSession<'dt> {
    pub name: &'dt str,
    pub key: &'dt str,
    pub subscriber: u32,
}

lastfm_t!(
    session,
    GetMobileSession,
    _GetMobileSession,
    Params,
    GetMobileSession,
    [username: &'rq str, password: &'rq str]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct GetSession<'dt> {
    pub name: &'dt str,
    pub key: &'dt str,
    pub subscriber: u32,
}

lastfm_t!(
    session,
    GetSession,
    _GetSession,
    Params,
    GetSession,
    [token: &'rq str]
);

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct GetToken<'dt> {
    pub token: &'dt str,
}

unwrapped_lastfm_t!(
    GetToken,
    Params,
    GetToken,
    []
);
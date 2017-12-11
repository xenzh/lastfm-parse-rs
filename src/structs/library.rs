use std::convert::Into;

use url::Url as StdUrl;

use lastfm_type::{LastfmType, Request, RequestParams};
use super::common::{Url, Image, str_to_val};

// ----------------------------------------------------------------

#[derive(Debug)]
pub enum Params<'pr> {
    GetArtists {
        user: &'pr str,
        limit: Option<u32>,
        page: Option<u32>,
    },
}

impl<'pr> RequestParams for Params<'pr> {
    fn method(&self) -> &str {
        match *self {
            Params::GetArtists { .. } => "library.getartists"
        }
    }

    fn append_to(&self, url: &mut StdUrl) {
        let mut query = url.query_pairs_mut();
        
        match *self {
            Params::GetArtists { user, limit, page } => {
                params!(query, [user: user], [limit: cv!(limit), page: cv!(page)]);
            }
        }
    }
}

// ----------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Artist<'dt> {
    pub name: &'dt str,
    pub mbid: Option<&'dt str>,
    url: Url<'dt>,
    #[serde(deserialize_with = "str_to_val")]
    pub playcount: u32,
    #[serde(deserialize_with = "str_to_val")]
    pub tagcount: u32,
    #[serde(deserialize_with = "str_to_val")]
    pub streamable: u32,
    pub image: Vec<Image<'dt>>,
}

#[derive(Deserialize, Debug)]
pub struct GetArtists<'dt> {
    #[serde(borrow)]
    pub artist: Option<Vec<Artist<'dt>>>,
}

lastfm_t!(
    artists,
    GetArtists,
    _Artists,
    Params,
    GetArtists,
    [user: &'rq str, limit: Option<u32>, page: Option<u32>]
);

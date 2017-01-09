extern crate hyper;
extern crate serde_json;

use std::io::Read;

use self::hyper::error::Result as HttpResult;
use self::hyper::client::Client;
use self::hyper::Url;

use super::{lastfm_obj_from_json, Result, Error};

use super::api_error::{ApiError, ApiErrorKind};
use super::album::Info as AlbumInfo;
use super::artist::{Info as ArtistInfo, SimilarArtists as ArtistsList};
use super::tag::{Info as TagInfo, Refs as TagList, TopRefs as TopTagList};
use super::track::Info as TrackInfo;
use super::user::Info as UserInfo;

// ----------------------------------------------------------------

type QueryPairs<'a> = Vec<(&'a str, &'a str)>;

fn make_lastfm_url(query_pairs: &QueryPairs) -> HttpResult<Url> {
    let mut url = Url::parse("http://ws.audioscrobbler.com/2.0/")?;
    url.query_pairs_mut().extend_pairs(query_pairs.into_iter());
    Ok(url)
}

fn get_content(url: &Url) -> HttpResult<String> {
    let client = Client::new();
    let mut rsp = client.get(url.clone()).send()?;

    let mut res = String::new();
    rsp.read_to_string(&mut res)?;
    Ok(res)
}

fn get_lastfm_obj_raw<'a>(api_key: &'a str,
                          method: &'a str,
                          params: Option<QueryPairs<'a>>)
                          -> HttpResult<String> {
    let mut params = match params {
        Some(vc) => vc,
        None => Vec::new(),
    };

    params.push(("api_key", api_key));
    params.push(("method", method));
    params.push(("format", "json"));

    let url = make_lastfm_url(&params)?;
    get_content(&url)
}

// ----------------------------------------------------------------

macro_rules! test_fn {
    ($title:ident, $method:expr, $entity:ty, $params:expr) => {
        #[test]
        fn $title() {
            let api_key = "143f59fafebb6ba4bbfafc6af666e1d6";
            let json_str = get_lastfm_obj_raw(api_key, $method, $params).unwrap();
            let data: $entity = lastfm_obj_from_json(&json_str).unwrap();
            println!("Deserialized \"{}\":\n{:?}\n", stringify!($entity), data);
        }
    };
}

test_fn!(deserialize_album_info,
         "album.getinfo",
         AlbumInfo,
         Some(vec![("artist", "iamthemorning"), ("album", "~")]));

test_fn!(deserialize_album_user_tags,
         "album.gettags",
         TagList,
         Some(vec![("artist", "amiina"), ("album", "fantomas"), ("user", "xenzh")]));

test_fn!(deserialize_album_top_tags,
         "album.gettoptags",
         TopTagList,
         Some(vec![("artist", "paavoharju"), ("album", "yhä hämärää")]));


test_fn!(deserialize_artist_info,
         "artist.getinfo",
         ArtistInfo,
         Some(vec![("artist", "schtimm")]));

test_fn!(deserialize_artist_user_tags,
         "artist.gettags",
         TagList,
         Some(vec![("artist", "adam lane"), ("user", "xenzh")]));

test_fn!(deserialize_artist_top_tags,
         "artist.gettoptags",
         TopTagList,
         Some(vec![("artist", "clann lir")]));

test_fn!(deserialize_artist_similar_artists,
         "artist.getsimilar",
         ArtistsList,
         Some(vec![("artist", "jill tracy"), ("limit", "3")]));


test_fn!(deserialize_tag_info,
         "tag.getinfo",
         TagInfo,
         Some(vec![("tag", "free jazz")]));


test_fn!(deserialize_track_info,
         "track.getinfo",
         TrackInfo,
         Some(vec![("artist", "strawfoot"), ("track", "the lord's wrath")]));

test_fn!(deserialize_track_user_tags,
         "track.gettags",
         TagList,
         Some(vec![("artist", "skalpel"), ("track", "test drive"), ("user", "xenzh")]));

test_fn!(deserialize_track_top_tags,
         "track.gettoptags",
         TopTagList,
         Some(vec![("artist", "vołosi"), ("track", "tsavkisi")]));


test_fn!(deserialize_user_info,
         "user.getinfo",
         UserInfo,
         Some(vec![("user", "xenzh")]));






fn get_api_error_json() -> String {
    let api_key = "143f59fafebb6ba4bbfafc6af666e1d6";
    // parameter error: in non-authenticated mode user should be specified for gettags
    let params = Some(vec![("artist", "the beatles"), ("album", "abbey road")]);
    get_lastfm_obj_raw(api_key, "album.gettags", params).unwrap()
}

#[test]
fn deserialize_api_error() {
    let json_str = get_api_error_json();
    let api_error: ApiError = serde_json::from_str(&json_str).unwrap();
    println!("Deserialized \"ApiError\":\n{:?}\n", api_error);
}

#[test]
fn capture_api_error() {
    let json_str = get_api_error_json();
    // TODO: change type to actual taglist once available
    let res: Result<TagInfo> = lastfm_obj_from_json(&json_str);

    assert!(res.is_err());

    match res.unwrap_err() {
        Error::Api(e) => assert_eq!(e.error, ApiErrorKind::InvalidParameters),
        _ => assert!(false),
    };
}

// ----------------------------------------------------------------

// temp for debug purposes, remove once done implementig data structures
// or maybe make some kind of lastfm-cli crate?..

use std::error::Error as StdError;
use std::io::Result as IoResult;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn dump_lastfm_obj<'a>(api_key: &'a str,
                       method: &'a str,
                       params: Option<QueryPairs<'a>>,
                       dump_path: &str)
                       -> IoResult<String> {
    let rsp = get_lastfm_obj_raw(api_key, method, params)
        .map_err( |e| IoError::new(IoErrorKind::Other, e.description()) )?;

    let path = Path::new(dump_path);
    let mut file = File::create(&path)?;
    file.write_all(rsp.as_bytes())?;
    Ok(rsp)
}

#[test]
//#[ignore]
fn dump_info() {
    let api_key = "143f59fafebb6ba4bbfafc6af666e1d6";
    let params = Some(vec![("artist", "vołosi"), ("limit", "2")]);
    let _raw_json = dump_lastfm_obj(api_key, "artist.getsimilar", params, "artist.gettags.json")
        .unwrap();
}
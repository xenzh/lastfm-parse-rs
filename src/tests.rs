extern crate hyper;
extern crate serde_json;

use std::io::Read;

use self::hyper::error::Result as HttpResult;
use self::hyper::client::Client;
use self::hyper::Url;

use super::album::Info as AlbumInfo;
use super::artist::Info as ArtistInfo;
use super::tag::Info as TagInfo;
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

fn get_lastfm_obj_raw<'a>(api_key: &'a str, method: &'a str, params: Option<QueryPairs<'a>>) -> HttpResult<String> {
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
    ($title:ident, $method:expr, $name:ident, $entity:ty, $params:expr) => {
        #[test]
        fn $title() {
            let api_key = "143f59fafebb6ba4bbfafc6af666e1d6";
            let json_str = get_lastfm_obj_raw(api_key, $method, $params).unwrap();
            let data = lastfm_obj_from_json!($entity, $name, json_str);
            println!("Deserialized \"{}\":\n{:?}\n", stringify!($entity), data);
        }
    };
}

test_fn!(
    deserialize_album_info,
    "album.getinfo",
    album,
    AlbumInfo,
    Some(vec![("artist", "iamthemorning"), ("album", "~")])
);

test_fn!(
    deserialize_artist_info,
    "artist.getinfo",
    artist,
    ArtistInfo,
    Some(vec![("artist", "schtimm")])
);

test_fn!(
    deserialize_tag_info,
    "tag.getinfo",
    tag,
    TagInfo,
    Some(vec![("tag", "free jazz")])
);

test_fn!(
    deserialize_track_info,
    "track.getinfo",
    track,
    TrackInfo,
    Some(vec![("artist", "strawfoot"), ("track", "the lord's wrath")])
);

test_fn!(
    deserialize_user_info,
    "user.getinfo",
    user,
    UserInfo,
    Some(vec![("user", "xenzh")])
);

// ----------------------------------------------------------------

// temp for debug purposes, remove once done implementig data structures
// or maybe make some kind of lastfm-cli crate?..

use std::error::Error;
use std::io::Result as IoResult;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn dump_lastfm_obj<'a>(api_key: &'a str, method: &'a str, params: Option<QueryPairs<'a>>, dump_path: &str) -> IoResult<String> {
    let rsp = get_lastfm_obj_raw(api_key, method, params).map_err( |e| IoError::new(IoErrorKind::Other, e.description()) )?;
    let path = Path::new(dump_path);
    let mut file = File::create(&path)?;
    file.write_all(rsp.as_bytes())?;
    Ok(rsp)
}

#[test]
#[ignore]
fn dump_info() {
    let api_key = "143f59fafebb6ba4bbfafc6af666e1d6";
    let params = Some(vec![("user", "xenzh")]);
    let _raw_json = dump_lastfm_obj(api_key, "user.getinfo", params, "user.json").unwrap();
}
extern crate hyper;
extern crate serde_json;

use std::io::Read;

use self::hyper::error::Result as HttpResult;
use self::hyper::client::Client;
use self::hyper::Url;

use super::{lastfm_obj_from_json, Result, Error};

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
    println!("Url: {}", url);
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

use super::tag::{Refs as TagList, TopRefs as TopTagList};


// Album

use super::album::Info as AlbumInfo;
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

use super::album::Search as AlbumSearch;
test_fn!(deserialize_album_search,
         "album.search",
         AlbumSearch,
         Some(vec![("album", "machine"), ("limit", "3")]));


// Artist

use super::artist::Info as ArtistInfo;
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

use super::artist::SimilarList as ArtistsList;
test_fn!(deserialize_artist_similar,
         "artist.getsimilar",
         ArtistsList,
         Some(vec![("artist", "jill tracy"), ("limit", "3")]));

use super::artist::Search as ArtistSearch;
test_fn!(deserialize_artist_search,
         "artist.search",
         ArtistSearch,
         Some(vec![("artist", "monteverdi"), ("limit", "3")]));

use super::artist::Corrections as ArtistCorrections;
test_fn!(deserialize_artist_corrections,
         "artist.getcorrection",
         ArtistCorrections,
         Some(vec![("artist", "blackmore")]));

use super::album::TopRefs as AlbumTopRefs;
test_fn!(deserialize_artist_top_albums,
         "artist.gettopalbums",
         AlbumTopRefs,
         Some(vec![("artist", "rainbow")]));

use super::track::TopRefs as TrackTopRefs;
test_fn!(deserialize_artist_top_tracks,
         "artist.gettoptracks",
         TrackTopRefs,
         Some(vec![("artist", "led zeppelin")]));


// Chart

use super::artist::ChartRefs as ChartArtistRefs;
test_fn!(deserialize_chart_top_artists,
         "chart.gettopartists",
         ChartArtistRefs,
         Some(vec![("limit", "5")]));

use super::tag::ChartRefs as ChartTagRefs;
test_fn!(deserialize_chart_top_tags,
         "chart.gettoptags",
         ChartTagRefs,
         Some(vec![("limit", "5")]));

use super::track::ChartRefs as ChartTrackRefs;
test_fn!(deserialize_chart_top_tracks,
         "chart.gettoptracks",
         ChartTrackRefs,
         Some(vec![("limit", "5")]));


// Geo

use super::artist::GeoRefs as ArtistGeoRefs;
test_fn!(deserialize_geo_top_artists,
         "geo.gettopartists",
         ArtistGeoRefs,
         Some(vec![("country", "ukraine"), ("limit", "5")]));

use super::track::GeoRefs as TrackGeoRefs;
test_fn!(deserialize_geo_top_tracks,
         "geo.gettoptracks",
         TrackGeoRefs,
         Some(vec![("country", "belarus"), ("limit", "5")]));


// Library

use super::artist::LibraryRefs as ArtistLibraryRefs;
test_fn!(deserialize_library_artists,
         "library.getartists",
         ArtistLibraryRefs,
         Some(vec![("user", "xenzh"), ("limit", "5")]));


// Tag

use super::tag::Info as TagInfo;
test_fn!(deserialize_tag_info,
         "tag.getinfo",
         TagInfo,
         Some(vec![("tag", "free jazz")]));

use super::tag::SimilarList as TagSimilarList;
test_fn!(deserialize_tag_similar,
         "tag.getsimilar",
         TagSimilarList,
         Some(vec![("tag", "death country")]));

use super::album::Refs as TagTopAlbumRefs;
test_fn!(deserialize_tag_top_albums,
         "tag.gettopalbums",
         TagTopAlbumRefs,
         Some(vec![("tag", "delta blues")]));

use super::artist::TopRefs as TagTopArtistRefs;
test_fn!(deserialize_tag_top_artists,
         "tag.gettopartists",
         TagTopArtistRefs,
         Some(vec![("tag", "freak cabaret")]));

use super::tag::GlobalRefs as TagTopTagRefs;
test_fn!(deserialize_tag_global_top_tags,
         "tag.gettoptags",
         TagTopTagRefs,
         Some(vec![]));

use super::track::TagTopRefs as TagTopTrackRefs;
test_fn!(deserialize_tag_top_tracks,
         "tag.gettoptracks",
         TagTopTrackRefs,
         Some(vec![]));

use super::chart::Weekly as TagWeeklyChartList;
test_fn!(deserialize_tag_weekly_chart_list,
         "tag.getweeklychartlist",
         TagWeeklyChartList,
         Some(vec![("tag", "field recording")]));


// Track

use super::track::Info as TrackInfo;
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

use super::track::SimilarList as TrackSimilarList;
test_fn!(deserialize_track_similar,
         "track.getsimilar",
         TrackSimilarList,
         Some(vec![("artist", "charles mingus"), ("track", "moanin'")]));

use super::track::Search as TrackSearch;
test_fn!(deserialize_track_search,
         "track.search",
         TrackSearch,
         Some(vec![("track", "fatalist"), ("limit", "3")]));

use super::track::Corrections as TrackCorrections;
test_fn!(deserialize_track_correction,
         "track.getcorrection",
         TrackCorrections,
         Some(vec![("artist", "guns and roses"), ("track", "Mrbrownstone")]));


// User

use super::user::Info as UserInfo;
test_fn!(deserialize_user_info,
         "user.getinfo",
         UserInfo,
         Some(vec![("user", "xenzh")]));


// ApiError

use super::api_error::{ApiError, ApiErrorKind};

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
    let params = Some(vec![("artist", "dsfhshsfgh"), ("track", "bargain")]);
    let _raw_json = dump_lastfm_obj(api_key, "track.getcorrection", params, "sample.json")
        .unwrap();
}
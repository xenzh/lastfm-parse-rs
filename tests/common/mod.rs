extern crate url;
extern crate serde_json;
extern crate async_http_client;

extern crate lastfm_parse_rs as lastfm;


use std::convert::TryFrom;

use self::url::Url;

use self::async_http_client::prelude::*;
use self::async_http_client::HttpRequest;

use self::lastfm::from_json_str;


static LASTFM_BASE_URL: &str = "http://ws.audioscrobbler.com/2.0/";
static LASTFM_API_KEY: &str = "INSERT_YOUR_API_KEY_HERE";

// ----------------------------------------------------------------

pub macro test_fn($name:ident, $lastfm_type:ident, [$($param_val:expr),*]) {
    #[test]
    fn $name() {
        let rq = $lastfm_type::request(LASTFM_BASE_URL, LASTFM_API_KEY, None, $($param_val),*);
        let url: Url = TryFrom::try_from(rq).unwrap();

        println!("\nUrl: {}\n", url);

        let req = HttpRequest::get(url).unwrap();
        let addr = req.addr().unwrap();
        
        let mut core = Core::new().unwrap(); 
        let handle = core.handle();

        let (res, _) = core.run(TcpStream::connect(&addr, &handle).and_then(|conn| {
            req.send(conn)
        })).unwrap();

        let res = res.unwrap();

        //temporary measure:
        // as of now serde doesnt support inplace escape sequence decoding
        // (see https://github.com/serde-rs/json/issues/318)
        // so we copying/replacing them manually
        let unescaped = String::from_utf8_lossy(res.get_body()).into_owned().replace(
            "\\\"",
            "'",
        );
        println!("\nRaw: {}\n", unescaped);

        let data: $lastfm_type = from_json_str(&unescaped).unwrap();
        println!("\nDeserialized {}:\n{:?}", stringify!($lastfm_type), data);
    }
}

extern crate url;
extern crate futures;
extern crate tokio_core;
extern crate hyper;
extern crate serde_json;

extern crate lastfm_parse_rs as lastfm;

use std::io::{Error as IoError, ErrorKind as IoErrorKind};

use self::futures::{Future, Stream};
use self::hyper::{Client, Uri, Chunk};
use self::tokio_core::reactor::Core;

use self::url::Url;
use self::lastfm::from_json_str;

static LASTFM_API_KEY: &str = "INSERT_YOUR_API_KEY_HERE";

// ----------------------------------------------------------------

pub macro test_fn($name:ident, $lastfm_type:ident, [$($param_val:expr),*]) {
    #[test]
    fn $name() {
        let base_url = "http://ws.audioscrobbler.com/2.0/";
        let rq = $lastfm_type::request(base_url, LASTFM_API_KEY, $($param_val),*);

        let url: Url = Into::into(rq);
        let uri: Uri = url.into_string().parse().unwrap();

        let mut core = Core::new().unwrap();
        let client = Client::new(&core.handle());

        println!("\nUrl: {}\n", uri);

        let work = client.get(uri).and_then(|res| {
            res.body().concat2().and_then(move |body: Chunk| {
                // temporary measure:
                // as of now serde doesnt support inplace escape sequences decoding
                // (see https://github.com/serde-rs/json/issues/318)
                // so we copying/replacing them manually
                let unescaped = String::from_utf8_lossy(&body).into_owned().replace(
                    "\\\"",
                    "'",
                );
                
                println!("\nRaw: {}\n", unescaped);

                let data: $lastfm_type = from_json_str(&unescaped).map_err(
                    |e| IoError::new(IoErrorKind::Other, e),
                )?;

                println!("\nDeserialized {}:\n{:?}", stringify!($lastfm_type), data);
                Ok(())
            })
        });

        core.run(work).unwrap();
    }
}

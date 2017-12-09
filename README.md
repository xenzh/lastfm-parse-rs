# lastfm-parse-rs
*lastfm-parse-rs* is a collection of serde-based data types for Lastfm API plus some functionality to form proper API requests. It doesn't come with a full-fledged client, but can be used to build one.

## Usage
* [API Reference](https://www.last.fm/api/intro)
* [Crate documentation](https://xenzh.github.io/lastfm-parse-rs/)
* [Examples](https://github.com/xenzh/lastfm-parse-rs/tree/master/examples)

Include following lines to your Cargo.toml in order to use this library:
```toml
[dependencies]
lastfm-parse-rs = { git = "https://github.com/xenzh/lastfm-parse-rs" }
```
Unfortinately it's not on [crates.io](https://crates.io/) yet since I rely on forked [async-http-client](https://github.com/xenzh/async-http-client) crate for the tests, and the fork is not merged back yet.

Library exposes a bunch of data types through `tag`, `track`, `user` etc modules and a couple of parser functions like `from_json_str()`. In addition, each of said types has associated `request()` function returning an object that can be used to form an API request for the type.

For convenience types are named after corresponding API methods: for instance, `artist.gettopalbums` method is represented by `lastfm_parse_rs::artist::GetTopAlbums` data structure.

## Example
Please note that example below uses Tokio-based [async-http-client](https://github.com/xenzh/async-http-client) crate, so it may look a bit unusual. But in fact any HTTP/1.1 client can be used, last time I've checked the library was not bound by any networking code, except for tests.
```rust
extern crate async_http_client;
extern crate lastfm_parse_rs;

use lastfm_parse_rs::from_json_slice;
use lastfm_parse_rs::structs::artist::GetInfo;

use async_http_client::prelude::*;
use async_http_client::HttpRequest;

fn main() {
    let base_url = "http://ws.audioscrobbler.com/2.0/";
    let api_key = "INSERT_YOUR_API_KEY_HERE";
    let get_info = GetInfo::request(base_url, api_key, "iamthemorning", None, Some(1), None, None);
    let req = HttpRequest::get(get_info.as_url()).unwrap();

    let mut core = Core::new().unwrap();
    let addr = req.addr().unwrap();
    let handle = core.handle();
    let (res, _) = core.run(TcpStream::connect(&addr, &handle).and_then(
        |conn| req.send(conn),
    )).unwrap();

    let res = res.unwrap();
    let data: GetInfo = from_json_slice(res.get_body()).unwrap();
}
```

## Status
All GET API methods except those that require authentification should be covered.

I'd expect some bugs here and there since methods were not really tested on all possible argument combinations.

I plan to work on a client library next and expect this one to be updated with auth-based stuff and POST requests support.

## License
MIT

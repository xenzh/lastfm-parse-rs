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
    let req = HttpRequest::get(get_info.as_url().as_str()).unwrap();

    println!("Url: {}\n\n", get_info.as_url().as_str());

    let mut core = Core::new().unwrap();
    let addr = req.addr().unwrap();
    let handle = core.handle();
    let (res, _) = core.run(TcpStream::connect(&addr, &handle).and_then(|conn| {
        req.send(conn)
    })).unwrap();

    let res = res.unwrap();
    let data: GetInfo = from_json_slice(res.get_body()).unwrap();

    println!("Deserialized: {:?}\n\n", data);
}
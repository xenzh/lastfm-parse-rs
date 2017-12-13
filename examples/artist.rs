extern crate url;
extern crate async_http_client;
extern crate lastfm_parse_rs;

use url::Url;

use lastfm_parse_rs::from_json_slice;
use lastfm_parse_rs::structs::artist::GetInfo;

use async_http_client::prelude::*;
use async_http_client::HttpRequest;


fn main() {
    let base_url = "http://ws.audioscrobbler.com/2.0/";
    let api_key = "INSERT_YOUR_API_KEY_HERE";
    let get_info = GetInfo::request(
        base_url,
        api_key,
        None,
        None,
        "iamthemorning",
        None,
        Some(1),
        None,
        None,
    );

    let url: Url = get_info.get_url().unwrap();
    println!("\nUrl: {}\n", url.as_str());
    
    let req = HttpRequest::get(url).unwrap();

    let mut core = Core::new().unwrap();
    let addr = req.addr().unwrap();
    let handle = core.handle();
    let (res, _) = core.run(TcpStream::connect(&addr, &handle).and_then(
        |conn| req.send(conn),
    )).unwrap();

    let res = res.unwrap();
    let data: Result<GetInfo, _> = from_json_slice(res.get_body());

    println!("Deserialized: {:?}\n", data);
}

#[derive(Debug)]
pub enum Method {
    TagGetInfo,
    TagGetSimilar,
    TagGetTopAlbums,
    TagGetTopArtists,
    TagGetTopTags,
    TagGetTopTracks,
    TagGetWeeklyChartList,
}

impl Method {
    pub fn api_name(&self) -> &'static str {
        match *self {
            Method::TagGetInfo => "tag.getinfo",
            Method::TagGetSimilar => "tag.getsimilar",
            Method::TagGetTopAlbums => "tag.gettopalbums",
            Method::TagGetTopArtists => "tag.gettopartists",
            Method::TagGetTopTags => "tag.gettoptags",
            Method::TagGetTopTracks => "tag.gettoptracks",
            Method::TagGetWeeklyChartList => "tag.getweeklychartlist",
        }
    }
}

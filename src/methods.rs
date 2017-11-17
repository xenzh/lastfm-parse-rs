#[derive(Debug)]
pub enum Method {
    ArtistAddTags,
    ArtistGetCorrection,
    ArtistGetInfo,
    ArtistGetSimilar,
    ArtistGetTags,
    ArtistGetTopAlbums,
    ArtistGetTopTags,
    ArtistGetTopTracks,
    ArtistRemoveTag,
    ArtistSearch,

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
            Method::ArtistAddTags => "artist.addtags",
            Method::ArtistGetCorrection => "artist.getcorrection",
            Method::ArtistGetInfo => "artist.getinfo",
            Method::ArtistGetSimilar => "artist.getsimilar",
            Method::ArtistGetTags => "artist.gettags",
            Method::ArtistGetTopAlbums => "artist.gettopalbums",
            Method::ArtistGetTopTags => "artist.gettoptags",
            Method::ArtistGetTopTracks => "artist.gettoptracks",
            Method::ArtistRemoveTag => "artist.removetag",
            Method::ArtistSearch => "artist.search",

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

#[derive(Debug)]
pub enum Method {
    AlbumAddTags,
    AlbumGetInfo,
    AlbumGetTags,
    AlbumGetTopTags,
    AlbumRemoveTag,
    AlbumSearch,

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

    TrackAddTags,
    TrackGetCorrection,
    TrackGetInfo,
    TrackGetSimilar,
    TrackGetTags,
    TrackGetTopTags,
    TrackLove,
    TrackRemoveTag,
    TrackScrobble,
    TrackSearch,
    TrackUnlove,
    TrackUpdateNowPlaying,

    UserGetArtistTracks,
    UserGetFriends,
    UserGetInfo,
    UserGetLovedTracks,
    UserGetPersonalTags,
    UserGetRecentTracks,
    UserGetTopAlbums,
    UserGetTopArtists,
    UserGetTopTags,
    UserGetTopTracks,
    UserGetWeeklyAlbumChart,
    UserGetWeeklyArtistChart,
    UserGetWeeklyChartList,
    UserGetWeeklyTrackChart,
}

impl Method {
    pub fn api_name(&self) -> &'static str {
        match *self {
            Method::AlbumAddTags => "album.addtags",
            Method::AlbumGetInfo => "album.getinfo",
            Method::AlbumGetTags => "album.gettags",
            Method::AlbumGetTopTags => "album.gettoptags",
            Method::AlbumRemoveTag => "album.removetag",
            Method::AlbumSearch => "album.search",

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

            Method::TrackAddTags => "track.addtags",
            Method::TrackGetCorrection => "track.getcorrection",
            Method::TrackGetInfo => "track.getinfo",
            Method::TrackGetSimilar => "track.getsimilar",
            Method::TrackGetTags => "track.gettags",
            Method::TrackGetTopTags => "track.gettoptags",
            Method::TrackLove => "track.love",
            Method::TrackRemoveTag => "track.removetags",
            Method::TrackScrobble => "track.scrobble",
            Method::TrackSearch => "track.search",
            Method::TrackUnlove => "track.unlove",
            Method::TrackUpdateNowPlaying => "track.updatenowplaying",

            Method::UserGetArtistTracks => "user.getartisttracks",
            Method::UserGetFriends => "user.getfriends",
            Method::UserGetInfo => "user.getinfo",
            Method::UserGetLovedTracks => "user.getlovedtracks",
            Method::UserGetPersonalTags => "user.getpersonaltags",
            Method::UserGetRecentTracks => "user.getrecenttracks",
            Method::UserGetTopAlbums => "user.gettopalbums",
            Method::UserGetTopArtists => "user.gettopartists",
            Method::UserGetTopTags => "user.gettoptags",
            Method::UserGetTopTracks => "user.gettoptracks",
            Method::UserGetWeeklyAlbumChart => "user.getweeklyalbumchart",
            Method::UserGetWeeklyArtistChart => "user.getweeklyartistchart",
            Method::UserGetWeeklyChartList => "user.getweeklychartlist",
            Method::UserGetWeeklyTrackChart => "user.getweeklytrackchart",
        }
    }
}

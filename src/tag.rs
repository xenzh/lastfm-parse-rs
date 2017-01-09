use std::convert::Into;
use super::common::{Wrapped, Url};

/// api methods: tag.getinfo
wrapper_t!(TagInfo, tag, Info);

/// api methods: album.gettags, artist.gettags, tack.gettags
wrapper_t!(TagList, tags, Refs);

/// apt methods: artist.gettoptags
wrapper_t!(TagTopList, toptags, TopRefs);

#[derive(Deserialize, Debug)]
pub struct Wiki {
    summary: String,
    content: String,
}

#[derive(Deserialize, Debug)]
pub struct Info {
    name: String,
    total: u32,
    reach: u32,
    wiki: Wiki,
}

#[derive(Deserialize, Debug)]
pub struct Ref {
    pub name: String,
    pub url: Url,
}

#[derive(Deserialize, Debug)]
pub struct Refs {
    pub tag: Option<Vec<Ref>>,
}

#[derive(Deserialize, Debug)]
pub struct TopRef {
    pub count: u32,
    pub name: String,
    pub url: Url,
}

#[derive(Deserialize, Debug)]
pub struct TopRefs {
    pub tag: Option<Vec<TopRef>>,
}
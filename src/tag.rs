use std::convert::Into;
use super::common::{Wrapped, Url};


/// api methods: tag.getinfo
wrapper_t!(TagInfo, tag, Info);

/// api methods: album.gettags, artist.gettags, tack.gettags
wrapper_t!(TagList, tags, Refs);

/// apt methods: artist.gettoptags
wrapper_t!(TagTopList, toptags, TopRefs);

/// api methods: tag.getsimilar
wrapper_t!(TagSimilarList, similartags, SimilarList);


#[derive(Deserialize, Debug)]
pub struct Wiki {
    pub summary: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct Info {
    pub name: String,
    pub total: u32,
    pub reach: u32,
    pub wiki: Wiki,
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

#[derive(Deserialize, Debug)]
pub struct Similar {
    pub name: String,
    pub url: Url,
    pub streamable: u32,
}

#[derive(Deserialize, Debug)]
pub struct SimilarList {
    pub tag: Option<Vec<Similar>>,
}
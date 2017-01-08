use super::common::Url;

#[derive(Deserialize, Debug)]
pub struct Wiki {
    summary: String,
    content: String,
}

/// api methods: tag.getinfo
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
    pub tag: Vec<Ref>,
}
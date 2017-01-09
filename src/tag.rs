use super::common::Url;

/// api methods: tag.getinfo
wrapper_t!(tag, Info);

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
    pub tag: Vec<Ref>,
}
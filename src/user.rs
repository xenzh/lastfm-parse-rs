use super::common::{Image, Url};

#[derive(Deserialize, Debug)]
pub enum Gender {
    #[serde(rename="m")]
    Male,
    #[serde(rename="f")]
    Female,
    #[serde(rename="n")]
    NotSpecified,
}

#[derive(Deserialize, Debug)]
pub struct Registered {
    // #text is omitted (I see it's the same as unixtime anyway)
    pub unixtime: String,
}

#[derive(Deserialize, Debug)]
pub struct Info {
    pub name: String,
    pub realname: String,
    pub image: Vec<Image>,
    pub url: Url,
    pub country: String,
    pub age: u32,
    pub gender: Gender,
    pub subscriber: u32,
    pub playcount: u32,
    pub bootstrap: u32,
    pub registered: Registered,
    #[serde(rename="type")]
    pub usertype: String, // no idea what other types are
}

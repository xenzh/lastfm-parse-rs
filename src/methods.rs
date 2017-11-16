#[derive(Debug)]
pub enum Method {
    TagGetInfo,
}

impl Method {
    pub fn api_name(&self) -> &'static str {
        match *self {
            Method::TagGetInfo => "tag.getinfo",
        }
    }
}

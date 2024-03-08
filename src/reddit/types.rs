#[derive(serde::Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum Thing {
    #[serde(rename = "t3")]
    Post(Post),
    #[serde(rename = "Listing")]
    Listing(Listing),
}

#[derive(serde::Deserialize)]
pub struct Listing {
    pub after: Option<String>,
    pub before: Option<String>,
    pub dist: u32,
    pub children: Vec<Thing>,
}

#[derive(serde::Deserialize)]
pub struct Post {
    pub title: String,
    pub subreddit: String,
    pub subreddit_name_prefixed: String,
    pub author: Option<String>,
    pub url: String,
    pub thumbnail: Option<String>,
    pub preview: Option<Preview>,
}

#[derive(serde::Deserialize)]
pub struct Preview {
    pub images: Vec<Image>,
}

#[derive(serde::Deserialize)]
pub struct Image {
    pub id: String,
    pub source: Source,
    pub resolutions: Vec<Source>,
}

#[derive(serde::Deserialize)]
pub struct Source {
    pub url: String,
    pub width: i32,
    pub height: i32,
}

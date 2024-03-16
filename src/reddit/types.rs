use std::collections::HashMap;

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
    pub dist: u64,
    pub children: Vec<Thing>,
}

#[derive(serde::Deserialize)]
pub struct Post {
    pub id: String,
    /// `id` with "t2_" prefix
    pub name: String,
    pub title: String,
    pub subreddit: String,
    pub subreddit_name_prefixed: String,
    pub subreddit_subscribers: Option<u64>,
    pub author: Option<String>,

    pub url: String,
    pub url_overridden_by_dest: Option<String>,

    pub selftext: Option<String>,
    pub domain: Option<String>,
    pub thumbnail: Option<String>,
    pub preview: Option<Preview>,
    #[serde(flatten)]
    pub gallery: Option<Gallery>,

    pub num_comments: u64,
    pub num_crossposts: u64,

    pub downs: i64,
    pub ups: i64,
    /// `ups` - `downs`
    pub score: i64,
    /// `ups` / `ups` + `downs`
    pub upvote_ratio: f64,

    /// None = No votes, Some(true) = upvote, Some(false) = downvote
    pub likes: Option<bool>,
    pub saved: bool,

    pub hidden: bool,
    pub spoiler: bool,
    pub pinned: bool,

    pub over_18: bool,
}

#[derive(serde::Deserialize)]
pub struct Preview {
    pub images: Vec<PreviewSourceSet>,
    pub enabled: bool,
}

#[derive(serde::Deserialize)]
pub struct Gallery {
    pub gallery_data: GalleryData,
    pub media_metadata: HashMap<String, SourceSet>,
}

#[derive(serde::Deserialize)]
pub struct GalleryData {
    pub items: Vec<GalleryItem>,
}

#[derive(serde::Deserialize)]
pub struct GalleryItem {
    pub id: u64,
    pub media_id: String,
    pub caption: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct PreviewSourceSet {
    #[serde(flatten)]
    pub default: SourceSet,
    pub variants: Option<ImageVariants>,
}

#[derive(serde::Deserialize)]
pub struct SourceSet {
    pub id: String,
    #[serde(alias = "s")]
    pub source: Source,
    #[serde(alias = "p")]
    pub resolutions: Vec<Source>,
}

#[derive(serde::Deserialize)]
pub struct ImageVariants {
    pub gif: Option<SourceSet>,
    pub mp4: Option<SourceSet>,
}

#[derive(serde::Deserialize)]
pub struct Source {
    #[serde(alias = "u")]
    pub url: String,
    #[serde(alias = "x")]
    pub width: i64,
    #[serde(alias = "y")]
    pub height: i64,
}

use crate::constants::*;
use reqwest::Url;

pub mod html;
pub mod types;

pub fn login_url(state: Option<String>) -> String {
    let mut url = Url::parse_with_params(
        "https://www.reddit.com/api/v1/authorize?response_type=code&duration=permanent",
        [
            ("client_id", CLIENT_ID.get().unwrap().as_str()),
            ("redirect_uri", REDIRECT_URI.get().unwrap().as_str()),
            (
                "scope",
                "identity history mysubreddits read save subscribe vote",
            ),
        ],
    )
    .expect("login url to be well formed");
    if let Some(s) = state {
        url.query_pairs_mut().append_pair("state", &s);
    } else {
        url.query_pairs_mut().append_pair("state", "/");
    }
    url.to_string()
}

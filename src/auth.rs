use crate::constants::*;
use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use serde::Deserialize;

pub fn router() -> Router {
    Router::new()
        .route("/oauth", get(oauth))
        .route("/logout", get(logout))
}

#[derive(Deserialize)]
struct OAuthQuery {
    error: Option<String>,
    code: Option<String>,
    state: Option<String>,
}
#[derive(Deserialize)]
struct OAuthResponse {
    access_token: String,
    expires_in: u32,
    scope: Option<String>,
    refresh_token: Option<String>,
}
async fn oauth(
    Query(auth): Query<OAuthQuery>,
    cookiejar: CookieJar,
) -> axum::response::Result<impl IntoResponse> {
    match auth {
        OAuthQuery {
            error: Some(error), ..
        } => Err((StatusCode::BAD_REQUEST, error))?,
        OAuthQuery {
            code: Some(code),
            state,
            ..
        } => {
            let res = match reqwest::Client::new()
                .post("https://www.reddit.com/api/v1/access_token")
                .basic_auth(CLIENT_ID.get().unwrap(), CLIENT_SECRET.get())
                .form(&[
                    ("grant_type", "authorization_code"),
                    ("code", code.as_str()),
                    ("redirect_uri", REDIRECT_URI.get().unwrap().as_str()),
                ])
                .send()
                .await
            {
                Ok(res) => res,
                Err(err) => Err(err.to_string())?,
            };
            let Ok(auth) = res.json::<OAuthResponse>().await else {
                Err("Error parsing oauth response")?
            };
            // Wierd cookie shadowing
            let cookiejar = cookiejar.add(
                Cookie::build(("access_token", auth.access_token))
                    .http_only(true)
                    .secure(true)
                    .max_age(cookie::time::Duration::new(auth.expires_in as i64, 0)),
            );
            let cookiejar = if let Some(refresh_token) = auth.refresh_token {
                cookiejar.add(
                    Cookie::build(("refresh_token", refresh_token))
                        .http_only(true)
                        .secure(true),
                )
            } else {
                cookiejar
            };
            return Ok((cookiejar, Redirect::to(&state.unwrap_or("/".to_string()))));
        }
        _ => Err((StatusCode::BAD_REQUEST, "Missing code"))?,
    }
}

async fn logout(cookiejar: CookieJar) -> impl IntoResponse {
    (
        cookiejar.remove("access_token").remove("access_token"),
        Redirect::to("/"),
    )
}

use axum::{
    extract::{Query, RawQuery},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_extra::extract::{cookie::Cookie, cookie::Expiration, CookieJar, OptionalPath};
use maud::{html, Markup, DOCTYPE};
use reddit::types::Thing;
use tokio::sync::OnceCell;
use tower_http::services::ServeDir;

mod reddit;

const USER_AGENT: &str = "web:shuttle-redddit:0.0.1 (by u/steve_dark03)";

static CLIENT_ID: OnceCell<String> = OnceCell::const_new();
static CLIENT_SECRET: OnceCell<String> = OnceCell::const_new();
static REDIRECT_URI: OnceCell<String> = OnceCell::const_new();

fn init_secrets(secret_store: shuttle_secrets::SecretStore) {
    for (cell, key) in [
        (&CLIENT_ID, "PUBLIC_CLIENT_ID"),
        (&CLIENT_SECRET, "CLIENT_SECRET"),
        (&REDIRECT_URI, "PUBLIC_REDIRECT_URI"),
    ] {
        cell.set(
            secret_store
                .get(key)
                .expect(&format!("{key} to exist in Secrets")),
        )
        .expect("PUBLIC_REDIRECT_URI to exist in Secrets")
    }
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    init_secrets(secret_store);

    let router = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/test", get(test))
        .route("/oauth", get(oauth))
        .route("/logout", get(logout))
        .route("/", get(reddit))
        .route("/*path", get(reddit));

    Ok(router.into())
}

struct Test {
    name: String,
}

impl Test {
    fn render(&self) -> Markup {
        html! {
            div class="bg-red-500" {
                (self.name)
            }
            test-element test=(self.name) {}
        }
    }
}
async fn test() -> impl IntoResponse {
    let t = Test {
        name: "Yeyetus".to_string(),
    };
    doc_tempalte(html! {
        (t.render())
    })
}

#[derive(serde::Deserialize)]
struct OAuthQuery {
    error: Option<String>,
    code: Option<String>,
    state: Option<String>,
}
#[derive(serde::Deserialize)]
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
                .header("User-Agent", USER_AGENT)
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

async fn reddit(
    OptionalPath(path): OptionalPath<String>,
    RawQuery(query): RawQuery,
    cookiejar: CookieJar,
) -> axum::response::Result<impl IntoResponse> {
    dbg!(&path, &query);
    let req = reqwest::Client::new()
        .get(format!(
            "https://oauth.reddit.com/{}.json?raw_json=1&{}",
            path.clone().unwrap_or_default(),
            query.unwrap_or_default()
        ))
        .header("User-Agent", USER_AGENT);
    let access_token = cookiejar.get("access_token");
    let req = if let Some(token) = access_token {
        req.bearer_auth(token.value())
    } else {
        req
    };
    let res = req.send().await.map_err(|err| err.to_string())?;

    let Thing::Listing(listing) = res.json().await.map_err(|err| err.to_string())? else {
        Err("Did not get a listing from reddit".to_string())?
    };

    Ok(reddit_template(
        html! {
            #posts class="space-y-4 p-4" {
                @for thing in &listing.children {
                    @match thing {
                        Thing::Post(post) =>{
                            div class="rounded border bg-slate-800 p-2 text-slate-100" {
                                a href=(post.url) {(post.title)}
                                div class="flex gap-2" {
                                    @if let Some(author) = &post.author {
                                        span {(author)}
                                    } @else {
                                        i { "removed" }
                                    }
                                    a href=(post.subreddit_name_prefixed) {
                                        (post.subreddit)
                                    }
                                }
                                @if let Some(preview) = &post.preview {
                                    @let image = preview.images.first().unwrap();
                                    img class="m-auto" src=(image.source.url)
                                    width=(image.source.width)
                                    height=(image.source.height);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        },
        access_token.is_none().then_some(path),
    ))
}

fn reddit_template(content: Markup, path: Option<Option<String>>) -> Markup {
    doc_tempalte(html! {
        header class="sticky top-0 flex justify-between bg-slate-800 p-2 text-slate-100" {
            "Reddit"
            @if let Some(p) = path {
                a href=(&reddit::login_url(p)) { "Login" }
            } @else {
                a href=("/logout") { "Logout" }
            }
        }
        (content)
    })
}

fn doc_tempalte(content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "Test" }
                link href="/assets/css/output.css" rel="stylesheet";
                (js_scripts())
                (js_components())
            }
            body hx-boost="true" {(content)}
        }
    }
}

fn js_scripts() -> Markup {
    html! {
        script type="text/javascript" src="/assets/js/htmx.js" {}
        script type="text/javascript" src="/assets/js/van-1.3.0.nomodule.min.js" {}
        script type="text/javascript" src="/assets/js/van-element.browser.js" {}
        script type="text/javascript" src="/assets/js/van-x.nomodule.min.js" {}
    }
}

fn js_components() -> Markup {
    html! {
        script type="module" src="/assets/components/test.js" {}
    }
}

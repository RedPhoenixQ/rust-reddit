use axum::{extract::RawQuery, response::IntoResponse, routing::get, Extension, Router};
use axum_extra::extract::{CookieJar, OptionalPath};
use maud::{html, Markup, DOCTYPE};
use reddit::types::Thing;
use reqwest::Client;
use tower_http::services::ServeDir;
use tracing::info;

mod auth;
mod constants;
mod reddit;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    constants::init(secret_store);

    let router = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .merge(auth::router())
        .route("/test", get(test))
        .route("/", get(reddit))
        .route("/*path", get(reddit))
        .layer(Extension(
            Client::builder()
                .user_agent(constants::USER_AGENT)
                .build()
                .expect("Reqwest client to be created"),
        ));

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

async fn reddit(
    Extension(client): Extension<Client>,
    OptionalPath(path): OptionalPath<String>,
    RawQuery(query): RawQuery,
    cookiejar: CookieJar,
) -> axum::response::Result<impl IntoResponse> {
    info!("Reddit request with path {:?}", path);
    let req = client.get(format!(
        "https://oauth.reddit.com/{}.json?raw_json=1&{}",
        path.clone().unwrap_or_default(),
        query.unwrap_or_default()
    ));
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
                        Thing::Post(post) => {
                            (post.render_feed())
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
        header class="sticky top-0 z-30 flex justify-between bg-slate-800 p-2 text-slate-100" {
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
        script type="module" src="/assets/components/gallery.js" {}
    }
}

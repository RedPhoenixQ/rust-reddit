use tokio::sync::OnceCell;

pub const USER_AGENT: &str = "web:shuttle-redddit:0.0.1 (by u/steve_dark03)";

pub static CLIENT_ID: OnceCell<String> = OnceCell::const_new();
pub static CLIENT_SECRET: OnceCell<String> = OnceCell::const_new();
pub static REDIRECT_URI: OnceCell<String> = OnceCell::const_new();

pub fn init(secret_store: shuttle_secrets::SecretStore) {
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

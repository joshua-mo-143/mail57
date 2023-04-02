use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;

mod auth;
mod database;

use auth::Auth;

#[derive(Clone)]
pub struct AppState {
    postgres: PgPool,
    key: Key,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres] postgres: PgPool,
    #[shuttle_secrets::Secrets] _secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&postgres)
        .await
        .expect("Migrations didn't work :(");

    let state = AppState {
        postgres,
        key: Key::generate(),
    };

    let router = Auth::router(state);

    Ok(router.into())
}

fn get_secrets(_secrets: SecretStore) {}

use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    routing::post,
    Json, Router,
};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::PrivateCookieJar;
use chrono::{Duration, Utc};
use time::Duration as TimeDuration;

pub struct Auth;

use crate::database::{Db, LoginDetails, RegisterDetails};
use crate::AppState;

impl Auth {
    pub fn router(state: AppState) -> Router {
        Router::new()
            .route("/register", post(Auth::register))
            .route("/login", post(Auth::login))
            .with_state(state)
    }

    async fn register(
        State(state): State<AppState>,
        Json(req): Json<RegisterDetails>,
    ) -> Result<StatusCode, StatusCode> {
        let hashed_password = bcrypt::hash(req.password, 10).unwrap();

        let create_user =
            Db::create_user(state.postgres, req.username, req.email, hashed_password).await;

        let Ok(_) = create_user else {
			return Err(StatusCode::INTERNAL_SERVER_ERROR)
		};

        Ok(StatusCode::OK)
    }

    async fn login(
        State(state): State<AppState>,
        jar: PrivateCookieJar,
        Json(req): Json<LoginDetails>,
    ) -> Result<(PrivateCookieJar, StatusCode), StatusCode> {
        let Ok(user) = Db::get_user(state.postgres.clone(), req.username).await else {
			return Err(StatusCode::BAD_REQUEST)
		};

        if bcrypt::verify(req.password, &user.password).is_err() {
            return Err(StatusCode::BAD_REQUEST);
        };

        let session_id: String = "hehe".into();

        let maxage = Utc::now() + Duration::hours(1);

        let cookie = Cookie::build("sessionid", session_id.clone())
            .path("/")
            .secure(true)
            .http_only(true)
            .max_age(TimeDuration::HOUR)
            .finish();

        let Ok(_) = Db::create_session(state.postgres, session_id, user.id, maxage).await else {
			return Err(StatusCode::INTERNAL_SERVER_ERROR)
		};

        Ok((jar.add(cookie), StatusCode::OK))
    }

    async fn validate_session<B>(
        jar: PrivateCookieJar,
        State(state): State<AppState>,
        req: Request<B>,
        next: Next<B>,
    ) -> Result<StatusCode, StatusCode> {
        let Some(cookie) = jar.get("sessionid").map(|cookie| cookie.value().to_owned()) else {
            return Err(StatusCode::FORBIDDEN)
        };

        let query = sqlx::query("SELECT * FROM Sessions WHERE session_id = $1")
            .bind(cookie)
            .find_one(&state.postgres)
            .await;

        match query {
            Ok(_) => (jar, next.run(req).await),
            Err(_) => Err(StatusCode::FORBIDDEN),
        }
    }
}

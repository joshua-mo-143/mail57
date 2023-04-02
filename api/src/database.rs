use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize, sqlx::FromRow)]
pub struct LoginDetails {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterDetails {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, sqlx::FromRow)]
pub struct Campaign {
    name: String,
    desc: String,
    email_body: String,
}

pub struct Db;

impl Db {
    pub async fn get_user(
        conn: PgPool,
        username: String,
    ) -> Result<LoginDetails, Box<dyn std::error::Error>> {
        let query = sqlx::query_as::<_, LoginDetails>(
            "SELECT id, username, password FROM Users WHERE username = $1",
        )
        .bind(username)
        .fetch_one(&conn)
        .await
        .expect("Something went wrong! :(");

        Ok(query)
    }

    pub async fn create_user(
        conn: PgPool,
        username: String,
        email: String,
        password: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let query =
            sqlx::query("INSERT INTO Users (username, email, password) VALUES ($1, $2, $3)")
                .bind(username)
                .bind(email)
                .bind(password)
                .execute(&conn)
                .await;

        match query {
            Ok(_) => Ok(()),
            Err(_) => Err("ERR: Couldn't insert. Does this user already exist?".into()),
        }
    }

    pub async fn create_session(
        conn: PgPool,
        session_id: String,
        user_id: i32,
        expires: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let query = sqlx::query("INSERT INTO Sessions (user_id, session_id, expires) VALUES ($1, $2, $3) ON CONFLICT (user_id) DO SET (session_id, expires) = (EXCLUDED.session_id, EXCLUDED.expires)")
						.bind(user_id.to_string())
						.bind(session_id)
						.bind(expires)
						.execute(&conn)
						.await;

        match query {
            Ok(_) => Ok(()),
            Err(_) => Err("ERR: Couldn't insert, does a session already exist?".into()),
        }
    }

    pub async fn delete_session(
        conn: PgPool,
        username: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let query = sqlx::query(
            "DELETE FROM Sessions WHERE user_id = (SELECT id FROM Users WHERE username = $1)",
        )
        .bind(username)
        .execute(&conn)
        .await;

        match query {
            Ok(_) => Ok(()),
            Err(_) => Err("ERR: Nothing exists to delete!".into()),
        }
    }

    pub async fn get_campaigns(
        conn: PgPool,
        username: String,
    ) -> Result<Vec<Campaign>, Box<dyn std::error::Error>> {
        let query = sqlx::query_as::<_, Campaign>("SELECT name, desc, email_body FROM Campaigns WHERE owner_id = (SELECT id FROM Users WHERE username = $1)")
						.bind(username)
						.fetch_all(&conn)
						.await.expect("Something went wrong!");

        Ok(query)
    }
}

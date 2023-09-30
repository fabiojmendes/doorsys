use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

#[derive(Clone)]
pub struct UserRepository {
    pub pool: PgPool,
}

impl UserRepository {
    pub async fn fetch_one(&self, id: i64) -> Result<User, sqlx::Error> {
        sqlx::query_file_as!(User, "queries/user_select.sql", id,)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn fetch_all(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_file_as!(User, "queries/user_select_list.sql")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn update(&self, id: i64, new_user: &NewUser) -> Result<User, sqlx::Error> {
        sqlx::query_file_as!(
            User,
            "queries/user_update.sql",
            new_user.name,
            new_user.email,
            id,
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create(&self, new_user: &NewUser) -> Result<User, sqlx::Error> {
        sqlx::query_file_as!(
            User,
            "queries/user_insert.sql",
            new_user.name,
            new_user.email,
        )
        .fetch_one(&self.pool)
        .await
    }
}

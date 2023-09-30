use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CodeType {
    Pin,
    Fob,
    Unkown,
}

impl From<String> for CodeType {
    fn from(value: String) -> Self {
        match &*value {
            "pin" => Self::Pin,
            "fob" => Self::Fob,
            _ => Self::Unkown,
        }
    }
}

impl ToString for CodeType {
    fn to_string(&self) -> String {
        match self {
            Self::Pin => String::from("pin"),
            Self::Fob => String::from("fob"),
            Self::Unkown => String::from("unkown"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Code {
    pub code: String,
    pub user_id: i64,
    pub code_type: CodeType,
}

#[derive(Clone)]
pub struct CodeRepository {
    pub pool: PgPool,
}

impl CodeRepository {
    pub async fn create(&self, code: Code) -> Result<Code, sqlx::Error> {
        sqlx::query_file_as!(
            Code,
            "queries/code_insert.sql",
            &code.user_id,
            &code.code,
            &code.code_type.to_string(),
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update(&self, old_code: &str, new_code: &str) -> Result<Code, sqlx::Error> {
        sqlx::query_file_as!(Code, "queries/code_update.sql", new_code, old_code)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn fetch_all(&self, user_id: i64) -> Result<Vec<Code>, sqlx::Error> {
        sqlx::query_file_as!(Code, "queries/code_user_select.sql", user_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn fetch_one(&self, code: &str) -> Result<Code, sqlx::Error> {
        sqlx::query_file_as!(Code, "queries/code_select.sql", code)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn delete(&self, code: &str) -> Result<(), sqlx::Error> {
        sqlx::query_file!("queries/code_delete.sql", code)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

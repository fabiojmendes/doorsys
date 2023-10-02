use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum CodeType {
    Pin,
    Fob,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Code {
    pub code: String,
    pub customer_id: i64,
    pub code_type: CodeType,
}

#[derive(Clone)]
pub struct CodeRepository {
    pub pool: PgPool,
}

impl CodeRepository {
    pub async fn create(&self, code: Code) -> Result<Code, sqlx::Error> {
        sqlx::query_as!(
            Code,
            r#"
            insert into code (customer_id, code, code_type) values ($1, $2, $3) 
                returning customer_id, code, code_type as "code_type: _"
            "#,
            code.customer_id,
            code.code,
            code.code_type as _
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update(&self, old_code: &str, new_code: &str) -> Result<Code, sqlx::Error> {
        sqlx::query_as!(
            Code,
            r#"
            update code set code = $1 where code = $2
                returning customer_id, code, code_type as "code_type: _"
            "#,
            new_code,
            old_code,
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn fetch_all(&self, customer_id: i64) -> Result<Vec<Code>, sqlx::Error> {
        sqlx::query_as!(
            Code,
            r#"select code, customer_id, code_type as "code_type: _" from code where customer_id = $1"#,
            customer_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn fetch_one(&self, code: &str) -> Result<Code, sqlx::Error> {
        sqlx::query_as!(
            Code,
            r#"select code, customer_id, code_type as "code_type: _" from code where code = $1"#,
            code
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete(&self, code: &str) -> Result<(), sqlx::Error> {
        sqlx::query!("delete from code where code = $1", code)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

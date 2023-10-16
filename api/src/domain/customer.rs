use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCustomer {
    pub name: String,
    pub email: String,
}

#[derive(Clone)]
pub struct CustomerRepository {
    pub pool: PgPool,
}

impl CustomerRepository {
    pub async fn fetch_one(&self, id: i64) -> Result<Customer, sqlx::Error> {
        sqlx::query_as!(Customer, r#"select * from customer where id = $1"#, id,)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn fetch_all(&self) -> Result<Vec<Customer>, sqlx::Error> {
        sqlx::query_as!(Customer, r#"select * from customer"#)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn update(
        &self,
        id: i64,
        new_customer: &NewCustomer,
    ) -> Result<Customer, sqlx::Error> {
        sqlx::query_as!(
            Customer,
            r#"update customer set name = $1, email = $2 where id = $3 returning *"#,
            new_customer.name,
            new_customer.email,
            id,
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create(&self, new_customer: &NewCustomer) -> Result<Customer, sqlx::Error> {
        sqlx::query_as!(
            Customer,
            r#"insert into customer (name, email) values ($1, $2) returning *"#,
            new_customer.name,
            new_customer.email,
        )
        .fetch_one(&self.pool)
        .await
    }
}

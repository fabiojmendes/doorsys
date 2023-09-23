use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct UserCode {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub codes: Option<Vec<Code>>,
}

#[derive(Debug, Serialize, sqlx::Type)]
pub struct Code {
    pub id: i64,
    pub code: String,
}

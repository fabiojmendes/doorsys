use super::HttpResult;
use crate::domain::customer::{Customer, CustomerRepository, NewCustomer};
use axum::{
    extract::{Path, State},
    Json,
};

pub async fn create(
    State(customer_repo): State<CustomerRepository>,
    Json(customer_form): Json<NewCustomer>,
) -> HttpResult<Json<Customer>> {
    let customer = customer_repo.create(&customer_form).await?;
    Ok(Json(customer))
}

pub async fn update(
    State(customer_repo): State<CustomerRepository>,
    Path(id): Path<i64>,
    Json(new_customer): Json<NewCustomer>,
) -> HttpResult<Json<Customer>> {
    let customer = customer_repo.update(id, &new_customer).await?;
    Ok(Json(customer))
}

pub async fn get(
    State(customer_repo): State<CustomerRepository>,
    Path(id): Path<i64>,
) -> HttpResult<Json<Customer>> {
    let customer = customer_repo.fetch_one(id).await?;
    Ok(Json(customer))
}

pub async fn list(
    State(customer_repo): State<CustomerRepository>,
) -> HttpResult<Json<Vec<Customer>>> {
    let customers = customer_repo.fetch_all().await?;
    Ok(Json(customers))
}

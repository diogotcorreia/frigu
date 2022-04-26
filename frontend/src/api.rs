use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

pub enum ApiError {
    ConnectionError,
    AppError(String),
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub seller_id: u32,
    pub seller_name: String,
    pub price: u32,
    pub stock: u32,
}

pub async fn list_products() -> Result<Vec<Product>, ApiError> {
    let resp = Request::get("/api/products").send().await.unwrap();

    if resp.ok() {
        // TODO handle errors
        let products: Vec<Product> = resp.json().await.unwrap();
        Ok(products)
    } else {
        // TODO handle errors
        Err(ApiError::ConnectionError)
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginPayload {
    pub phone: String,
    pub password: String,
}

pub async fn login(credentials: &LoginPayload) -> Result<(), ApiError> {
    let resp = Request::post("/api/login")
        .json(credentials)
        .expect("payload must be serializable to json")
        .send()
        .await
        .unwrap();

    if resp.ok() {
        // TODO handle errors and set cookie
        Ok(())
    } else {
        Err(ApiError::ConnectionError)
    }
}

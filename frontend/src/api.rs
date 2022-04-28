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

#[derive(Clone, Serialize)]
pub struct ProductPayload {
    pub id: Option<u32>,
    pub name: String,
    pub description: Option<String>,
    pub stock: u32,
    pub price: u32,
}

pub async fn insert_product(product: &ProductPayload) -> Result<Product, ApiError> {
    let resp = Request::post("/api/product")
        .json(product)
        .expect("payload must be serializable to json")
        .send()
        .await
        .unwrap();

    if resp.ok() {
        // TODO handle errors
        let product = resp.json().await.unwrap();
        Ok(product)
    } else {
        Err(ApiError::ConnectionError)
    }
}

#[derive(Clone, Serialize)]
pub struct PurchaseProductPayload {
    pub quantity: u32,
}

pub async fn purchase_product(
    product_id: u32,
    payload: &PurchaseProductPayload,
) -> Result<(), ApiError> {
    let resp = Request::post(&format!("/api/product/{}/purchase", product_id))
        .json(payload)
        .expect("payload must be serializable to json")
        .send()
        .await
        .unwrap();

    if resp.ok() {
        Ok(())
    } else {
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
        // TODO handle errors
        Ok(())
    } else {
        Err(ApiError::ConnectionError)
    }
}

#[derive(Deserialize, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,

    pub phone_number: String,
}

pub async fn user_info() -> Result<User, ApiError> {
    let resp = Request::get("/api/user/info").send().await.unwrap();

    if resp.ok() {
        // TODO handle errors
        Ok(resp.json().await.unwrap())
    } else {
        Err(ApiError::ConnectionError)
    }
}

pub async fn logout() -> Result<(), ApiError> {
    let resp = Request::get("/api/logout").send().await.unwrap();

    if resp.ok() {
        Ok(())
    } else {
        Err(ApiError::ConnectionError)
    }
}

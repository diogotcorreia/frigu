use std::fmt::Display;

use chrono::{DateTime, Local};
use gloo_net::http::{Request, Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub enum ApiError {
    ConnectionError,
    HttpBadRequest(String),
    HttpConflict(String),
    HttpUnauthorized(String),
    HttpForbidden(String),
    HttpNotFound(String),
    GenericError(String),
    JsonError,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ApiError::ConnectionError => write!(f, "Failed to connect to server"),
            ApiError::HttpBadRequest(msg) => write!(f, "Invalid data: {}", msg),
            ApiError::HttpConflict(msg) => write!(f, "Error: {}", msg),
            ApiError::HttpUnauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            ApiError::HttpForbidden(msg) => write!(f, "No permission: {}", msg),
            ApiError::HttpNotFound(msg) => write!(f, "Not found: {}", msg),
            ApiError::GenericError(msg) => write!(f, "Error: {}", msg),
            ApiError::JsonError => write!(f, "Failed to parse data from server"),
        }
    }
}

impl From<gloo_net::Error> for ApiError {
    fn from(_: gloo_net::Error) -> Self {
        ApiError::ConnectionError
    }
}

async fn handle_response<T: DeserializeOwned>(response: Response) -> Result<T, ApiError> {
    if response.ok() {
        response.json().await.map_err(|_| ApiError::JsonError)
    } else {
        let error_message = response
            .text()
            .await
            .map_err(|_| ApiError::GenericError("failed to parse response text".to_string()))?;

        let error = match response.status() {
            400 => ApiError::HttpBadRequest(error_message),
            401 => ApiError::HttpUnauthorized(error_message),
            403 => ApiError::HttpForbidden(error_message),
            404 => ApiError::HttpNotFound(error_message),
            409 => ApiError::HttpConflict(error_message),
            _ => ApiError::GenericError(error_message),
        };

        Err(error)
    }
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
    let resp = Request::get("/api/products").send().await?;

    handle_response(resp).await
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

    handle_response(resp).await
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

    handle_response(resp).await
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

    handle_response(resp).await
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub phone_number: String,
}

pub async fn user_info() -> Result<User, ApiError> {
    let resp = Request::get("/api/user/info").send().await.unwrap();

    handle_response(resp).await
}

pub async fn logout() -> Result<(), ApiError> {
    let resp = Request::get("/api/logout").send().await.unwrap();

    if resp.ok() {
        Ok(())
    } else {
        Err(ApiError::ConnectionError)
    }
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct Purchase {
    pub id: u32,
    pub buyer: User,
    pub product: Product,
    pub quantity: u32,
    pub unit_price: u32,
    pub date: DateTime<Local>,
    pub paid_date: Option<DateTime<Local>>,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct BuyerGroupedPurchases {
    pub buyer: User,
    pub amount_due: u32,
    pub purchases: Vec<Purchase>,
}

pub async fn list_purchases() -> Result<Vec<Purchase>, ApiError> {
    let resp = Request::get("/api/purchases/history").send().await.unwrap();

    handle_response(resp).await
}

pub async fn seller_summary() -> Result<Vec<BuyerGroupedPurchases>, ApiError> {
    let resp = Request::get("/api/purchases/seller-summary")
        .send()
        .await
        .unwrap();

    handle_response(resp).await
}

pub async fn pay_purchase(purchase_id: u32) -> Result<(), ApiError> {
    let resp = Request::post(&format!("/api/purchase/{}/pay", purchase_id))
        .send()
        .await
        .unwrap();

    handle_response(resp).await
}

#[derive(Serialize)]
struct PayPurchaseUserBulkPayload {
    count: u32,
}

pub async fn pay_purchase_user_bulk(buyer_id: u32, purchase_count: u32) -> Result<(), ApiError> {
    let resp = Request::post(&format!("/api/purchase/user/{}/pay", buyer_id))
        .json(&PayPurchaseUserBulkPayload {
            count: purchase_count,
        })
        .expect("payload must be serializable to json")
        .send()
        .await
        .unwrap();

    handle_response(resp).await
}

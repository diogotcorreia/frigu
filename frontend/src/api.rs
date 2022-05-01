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

#[derive(Clone, Deserialize, PartialEq)]
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

#[derive(Clone, Deserialize, PartialEq)]
pub struct Purchase {
    pub id: u32,
    pub buyer: User,
    pub product: Product,
    pub quantity: u32,
    pub unit_price: u32,
    // TODO how to represent dates?
    pub date: String,
    pub paid_date: Option<String>,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct BuyerGroupedPurchases {
    pub buyer: User,
    pub amount_due: u32,
    pub purchases: Vec<Purchase>,
}

pub async fn list_purchases() -> Result<Vec<Purchase>, ApiError> {
    let resp = Request::get("/api/purchases/history").send().await.unwrap();

    if resp.ok() {
        Ok(resp.json().await.unwrap())
    } else {
        Err(ApiError::ConnectionError)
    }
}

pub async fn seller_summary() -> Result<Vec<BuyerGroupedPurchases>, ApiError> {
    let resp = Request::get("/api/purchases/seller-summary")
        .send()
        .await
        .unwrap();

    if resp.ok() {
        Ok(resp.json().await.unwrap())
    } else {
        Err(ApiError::ConnectionError)
    }
}

pub async fn pay_purchase(purchase_id: u32) -> Result<(), ApiError> {
    let resp = Request::post(&format!("/api/purchase/{}/pay", purchase_id))
        .send()
        .await
        .unwrap();

    if resp.ok() {
        Ok(())
    } else {
        Err(ApiError::ConnectionError)
    }
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

    if resp.ok() {
        Ok(())
    } else {
        Err(ApiError::ConnectionError)
    }
}

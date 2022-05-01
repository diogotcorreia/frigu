use entity::product;
use entity::purchase;
use entity::user;
use sea_orm::prelude::*;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Serialize, Deserialize)]
pub struct ProductDto {
    pub(crate) id: Option<u32>,
    pub(crate) seller_id: Option<u32>,
    pub(crate) seller_name: Option<String>,
    pub(crate) stock: u32,
    pub(crate) price: u32,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
}

impl ProductDto {
    pub(crate) async fn from_entity(
        entity: product::Model,
        conn: &DatabaseConnection,
    ) -> Result<Self, AppError> {
        let seller = user::Entity::find_by_id(entity.seller)
            .one(conn)
            .await?
            .expect("seller of product must exist");
        Ok(Self {
            id: Some(entity.id),
            seller_id: Some(entity.seller),
            seller_name: Some(seller.name),
            stock: entity.stock,
            price: entity.price,
            name: entity.name,
            description: entity.description,
        })
    }
}

#[derive(Deserialize)]
pub struct LoginDto {
    pub(crate) phone: String,
    pub(crate) password: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserDto {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) phone_number: String,
}

impl UserDto {
    pub(crate) fn from_entity(entity: user::Model) -> Result<Self, AppError> {
        Ok(Self {
            id: entity.id,
            name: entity.name,
            phone_number: entity.phone_number,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct PurchaseDto {
    pub(crate) id: Option<u32>,
    pub(crate) buyer: Option<UserDto>,
    pub(crate) product: Option<ProductDto>,
    pub(crate) quantity: u32,
    pub(crate) unit_price: Option<u32>,
    pub(crate) date: Option<DateTimeUtc>,
    pub(crate) paid_date: Option<DateTimeUtc>,
}

impl PurchaseDto {
    pub(crate) async fn from_entity(
        entity: purchase::Model,
        conn: &DatabaseConnection,
    ) -> Result<Self, AppError> {
        let product = product::Entity::find_by_id(entity.product)
            .one(conn)
            .await?
            .expect("product of purchase must exist");
        let buyer = user::Entity::find_by_id(entity.buyer)
            .one(conn)
            .await?
            .expect("buyer of purchase must exist");
        Ok(Self {
            id: Some(entity.id),
            buyer: Some(UserDto::from_entity(buyer)?),
            product: Some(ProductDto::from_entity(product, conn).await?),
            quantity: entity.quantity,
            unit_price: Some(entity.unit_price),
            date: Some(entity.date),
            paid_date: entity.paid_date,
        })
    }
}

#[derive(Serialize)]
pub(crate) struct BuyerGroupedPurchasesDto {
    pub(crate) buyer: UserDto,
    pub(crate) amount_due: u32,
    pub(crate) purchases: Vec<PurchaseDto>,
}

use entity::product;
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

#[derive(Serialize)]
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

#[derive(Deserialize)]
pub struct PurchaseDto {
    pub(crate) quantity: u32,
}

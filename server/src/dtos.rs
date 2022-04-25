use entity::product;
use entity::user;
use sea_orm::prelude::*;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

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
    pub(crate) async fn from_entity(entity: product::Model, conn: &DatabaseConnection) -> Self {
        let seller = user::Entity::find_by_id(entity.seller)
            .one(conn)
            .await
            .expect("TODO: handle DbErr")
            .expect("seller of product must exist");
        Self {
            id: Some(entity.id),
            seller_id: Some(entity.seller),
            seller_name: Some(seller.name),
            stock: entity.stock,
            price: entity.price,
            name: entity.name,
            description: entity.description,
        }
    }
}

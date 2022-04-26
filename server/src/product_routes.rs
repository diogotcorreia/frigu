use axum::{extract, Extension, Json};
use axum_extra::extract::CookieJar;
use entity::{
    product::{self, Entity as Product},
    sea_orm,
};
use sea_orm::{prelude::*, DatabaseConnection, QueryOrder, Set};

use crate::dtos::ProductDto;
use crate::errors::AppError;

pub(crate) async fn list(
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<Json<Vec<ProductDto>>, AppError> {
    let entities = Product::find()
        .filter(product::Column::Stock.gt(0))
        .order_by_desc(product::Column::Stock)
        .all(conn)
        .await?;
    let mut dtos = Vec::with_capacity(entities.len());
    for entity in entities {
        dtos.push(ProductDto::from_entity(entity, conn).await?);
    }
    Ok(Json(dtos))
}

pub(crate) async fn insert(
    extract::Json(product_dto): extract::Json<ProductDto>,
    Extension(ref conn): Extension<DatabaseConnection>,
    jar: CookieJar,
) -> Result<Json<ProductDto>, AppError> {
    let seller_id = crate::jwt_helpers::get_login(&jar)?;
    // validate stock
    let stock = product_dto.stock;
    if stock == 0 {
        return Err(AppError::BadInput("stock must be greater than 0"));
    }
    // validate price
    let price = product_dto.price;
    if price == 0 {
        return Err(AppError::BadInput("price must be greater than 0"));
    }
    // validate name
    let name = product_dto.name.trim();
    if name.is_empty() {
        return Err(AppError::BadInput("name can't be empty"));
    }
    // validate description
    let description = product_dto.description.and_then(|s| {
        let trimmed = s.trim();
        if !trimmed.is_empty() {
            Some(trimmed.to_string())
        } else {
            None
        }
    });
    let product = product::ActiveModel {
        name: Set(name.to_string()),
        description: Set(description),
        seller: Set(seller_id),
        stock: Set(stock),
        price: Set(price),
        ..Default::default()
    };

    let product = product.insert(conn).await?;

    let new_product_dto = ProductDto::from_entity(product, conn).await?;

    Ok(Json(new_product_dto))
}

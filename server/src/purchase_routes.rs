use std::collections::HashMap;

use axum::{extract::Path, Extension, Json};
use axum_extra::extract::CookieJar;
use entity::{
    product,
    purchase::{self, Entity as Purchase},
    sea_orm, user,
};
use sea_orm::{
    prelude::*, DatabaseConnection, JoinType, QueryOrder, QuerySelect, Set, TransactionTrait,
};

use crate::dtos::{BuyerGroupedPurchasesDto, PurchaseDto, SellerSummaryDto};
use crate::errors::AppError;

pub(crate) async fn seller_summary(
    Extension(ref conn): Extension<DatabaseConnection>,
    jar: CookieJar,
) -> Result<Json<SellerSummaryDto>, AppError> {
    let seller_id = crate::jwt_helpers::get_login(&jar)?;

    // Sold products
    let entities = Purchase::find()
        .join(JoinType::InnerJoin, purchase::Relation::Product.def())
        .filter(product::Column::Seller.eq(seller_id))
        .filter(purchase::Column::PaidDate.is_null())
        .order_by_desc(purchase::Column::Date)
        .all(conn)
        .await?;
    let mut dtos = Vec::with_capacity(entities.len());
    // FIXME: converting this model to DTO will trigger 3 SQL queries for each entity
    for entity in entities {
        dtos.push(PurchaseDto::from_entity(entity, conn).await?);
    }

    // Grouped amount due per user
    let buyer_grouped_purchases: Vec<BuyerGroupedPurchasesDto> = dtos
        .iter()
        .fold(HashMap::new(), |mut acc, purchase| {
            let buyer_id = purchase.buyer.as_ref().expect("buyer must exist").id;
            match acc.get_mut(&buyer_id) {
                None => {
                    acc.insert(buyer_id, vec![purchase]);
                }
                Some(vec) => {
                    vec.push(purchase);
                }
            };
            acc
        })
        .iter()
        .map(|(_, buyer_purchases)| {
            let amount_due: u32 = buyer_purchases
                .iter()
                .map(|purchase| {
                    purchase.quantity * purchase.unit_price.expect("purchase must have unit price")
                })
                .sum();

            BuyerGroupedPurchasesDto {
                buyer: buyer_purchases
                    .first()
                    .expect("there must be at least one purchase")
                    .buyer
                    .as_ref()
                    .expect("buyer must exist")
                    .clone(),
                amount_due,
            }
        })
        .collect();

    Ok(Json(SellerSummaryDto {
        purchases: dtos,
        summary: buyer_grouped_purchases,
    }))
}

pub(crate) async fn purchase_history(
    Extension(ref conn): Extension<DatabaseConnection>,
    jar: CookieJar,
) -> Result<Json<Vec<PurchaseDto>>, AppError> {
    let buyer_id = crate::jwt_helpers::get_login(&jar)?;

    let entities = Purchase::find()
        .join(JoinType::InnerJoin, purchase::Relation::User.def())
        .filter(user::Column::Id.eq(buyer_id))
        .order_by_desc(purchase::Column::Date)
        .all(conn)
        .await?;
    let mut dtos = Vec::with_capacity(entities.len());
    // FIXME: converting this model to DTO will trigger 3 SQL queries for each entity
    for entity in entities {
        dtos.push(PurchaseDto::from_entity(entity, conn).await?);
    }
    Ok(Json(dtos))
}

pub(crate) async fn pay_purchase(
    Path(purchase_id): Path<u32>,
    Extension(ref conn): Extension<DatabaseConnection>,
    jar: CookieJar,
) -> Result<(), AppError> {
    let _seller_id = crate::jwt_helpers::get_login(&jar)?;

    let txn = conn.begin().await?;

    let purchase = Purchase::find_by_id(purchase_id)
        .one(conn)
        .await?
        .ok_or(AppError::NoSuchPurchase)?;

    let mut purchase: purchase::ActiveModel = purchase.into();

    let now = chrono::offset::Utc::now();
    purchase.paid_date = Set(Some(now));

    purchase.update(conn).await?;

    // TODO check if seller_id matches the product seller
    txn.commit().await?;
    Ok(())
}

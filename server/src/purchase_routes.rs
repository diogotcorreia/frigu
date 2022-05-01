use std::collections::HashMap;

use axum::{extract::Path, Extension, Json};
use axum_extra::extract::CookieJar;
use entity::{
    product,
    purchase::{self, Entity as Purchase},
    sea_orm, user,
};
use sea_orm::{
    prelude::*, DatabaseConnection, FromQueryResult, JoinType, QueryOrder, QuerySelect, Set,
    TransactionTrait, Unchanged,
};

use crate::dtos::{BuyerGroupedPurchasesDto, PurchaseDto};
use crate::errors::AppError;

pub(crate) async fn seller_summary(
    Extension(ref conn): Extension<DatabaseConnection>,
    jar: CookieJar,
) -> Result<Json<Vec<BuyerGroupedPurchasesDto>>, AppError> {
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

    // Grouped amount due and purchases per user
    let buyer_grouped_purchases: Vec<BuyerGroupedPurchasesDto> = dtos
        .into_iter()
        .fold(
            HashMap::new(),
            |mut acc: HashMap<u32, Vec<PurchaseDto>>, purchase| {
                let buyer_id = purchase.buyer.as_ref().expect("buyer must exist").id;
                acc.entry(buyer_id).or_insert(Vec::new()).push(purchase);
                acc
            },
        )
        .into_iter()
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
                purchases: buyer_purchases,
            }
        })
        .collect();

    Ok(Json(buyer_grouped_purchases))
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

#[derive(Debug, FromQueryResult)]
struct PurchaseWithSeller {
    id: u32,
    seller_id: u32,
    paid_date: Option<DateTimeUtc>,
}

pub(crate) async fn pay_purchase(
    Path(purchase_id): Path<u32>,
    Extension(ref conn): Extension<DatabaseConnection>,
    jar: CookieJar,
) -> Result<(), AppError> {
    let seller_id = crate::jwt_helpers::get_login(&jar)?;

    let txn = conn.begin().await?;

    let purchase = Purchase::find_by_id(purchase_id)
        .column_as(product::Column::Seller, "seller_id")
        .join(JoinType::InnerJoin, purchase::Relation::Product.def())
        .into_model::<PurchaseWithSeller>()
        .one(&txn)
        .await?
        .ok_or(AppError::NoSuchPurchase)?;

    if purchase.seller_id != seller_id {
        return Err(AppError::Forbidden);
    }
    if purchase.paid_date.is_some() {
        return Err(AppError::PurchaseAlreadyPaid);
    }

    let now = chrono::offset::Utc::now();
    let purchase = purchase::ActiveModel {
        id: Unchanged(purchase.id),
        paid_date: Set(Some(now)),
        ..Default::default()
    };

    purchase.save(&txn).await?;

    txn.commit().await?;
    Ok(())
}

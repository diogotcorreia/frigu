use std::collections::HashMap;

use axum::{
    extract::{self, Path},
    Extension, Json,
};
use axum_extra::extract::CookieJar;
use entity::{
    product,
    purchase::{self, Entity as Purchase},
    sea_orm, user,
};
use migration::{Expr, Query};
use sea_orm::{
    prelude::*, DatabaseConnection, FromQueryResult, JoinType, QueryOrder, QuerySelect, Set,
    TransactionTrait, Unchanged,
};

use crate::{dtos::{BuyerGroupedPurchasesDto, PayPurchaseUserBulkDto, PurchaseDto}, Config};
use crate::errors::AppError;

pub(crate) async fn seller_summary(
    Extension(ref conn): Extension<DatabaseConnection>,
    Extension(ref config): Extension<Config>,
    jar: CookieJar,
) -> Result<Json<Vec<BuyerGroupedPurchasesDto>>, AppError> {
    let seller_id = crate::jwt_helpers::get_login(&jar, &config.hmac_secret)?;

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
    Extension(ref config): Extension<Config>,
    jar: CookieJar,
) -> Result<Json<Vec<PurchaseDto>>, AppError> {
    let buyer_id = crate::jwt_helpers::get_login(&jar, &config.hmac_secret)?;

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
    Extension(ref config): Extension<Config>,
    jar: CookieJar,
) -> Result<(), AppError> {
    let seller_id = crate::jwt_helpers::get_login(&jar, &config.hmac_secret)?;

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

pub(crate) async fn pay_purchase_user_bulk(
    Path(buyer_id): Path<u32>,
    extract::Json(action_dto): extract::Json<PayPurchaseUserBulkDto>,
    Extension(ref conn): Extension<DatabaseConnection>,
    Extension(ref config): Extension<Config>,
    jar: CookieJar,
) -> Result<(), AppError> {
    let seller_id = crate::jwt_helpers::get_login(&jar, &config.hmac_secret)?;

    let txn = conn.begin().await?;

    let now = chrono::offset::Utc::now();
    let purchase = purchase::ActiveModel {
        paid_date: Set(Some(now)),
        ..Default::default()
    };

    let result = Purchase::update_many()
        .set(purchase)
        .filter(purchase::Column::Buyer.eq(buyer_id))
        .filter(
            purchase::Column::Product.in_subquery(
                Query::select()
                    .expr(Expr::col(product::Column::Id))
                    .from(product::Entity)
                    .and_where(Expr::col(product::Column::Seller).eq(seller_id))
                    .to_owned(),
            ),
        )
        .filter(purchase::Column::PaidDate.is_null())
        .exec(&txn)
        .await?;

    if result.rows_affected != action_dto.count {
        return Err(AppError::BulkCountMismatch);
    }

    txn.commit().await?;
    Ok(())
}

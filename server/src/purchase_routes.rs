use axum::{Extension, Json};
use axum_extra::extract::CookieJar;
use entity::{
    product,
    purchase::{self, Entity as Purchase},
    sea_orm, user,
};
use sea_orm::{prelude::*, DatabaseConnection, JoinType, QueryOrder, QuerySelect};

use crate::dtos::PurchaseDto;
use crate::errors::AppError;

pub(crate) async fn seller_summary(
    Extension(ref conn): Extension<DatabaseConnection>,
    jar: CookieJar,
) -> Result<Json<Vec<PurchaseDto>>, AppError> {
    let seller_id = crate::jwt_helpers::get_login(&jar)?;

    let entities = Purchase::find()
        .join(JoinType::InnerJoin, purchase::Relation::Product.def())
        .join(JoinType::InnerJoin, product::Relation::User.def())
        .filter(user::Column::Id.eq(seller_id))
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

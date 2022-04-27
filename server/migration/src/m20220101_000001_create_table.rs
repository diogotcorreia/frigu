use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(
                        ColumnDef::new(User::PhoneNumber)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::HashedPassword).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Product::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Product::Name).string().not_null())
                    .col(ColumnDef::new(Product::Description).string())
                    .col(ColumnDef::new(Product::Seller).unsigned().not_null())
                    .col(ColumnDef::new(Product::Stock).unsigned().not_null())
                    .col(ColumnDef::new(Product::Price).unsigned().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-product-seller")
                            .from(Product::Table, Product::Seller)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Purchase::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Purchase::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Purchase::Buyer).unsigned().not_null())
                    .col(ColumnDef::new(Purchase::Product).unsigned().not_null())
                    .col(ColumnDef::new(Purchase::Quantity).unsigned().not_null())
                    .col(ColumnDef::new(Purchase::UnitPrice).unsigned().not_null())
                    .col(ColumnDef::new(Purchase::Date).date_time().not_null())
                    .col(ColumnDef::new(Purchase::PaidDate).date_time())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-purchase-buyer")
                            .from(Purchase::Table, Purchase::Buyer)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-purchase-product")
                            .from(Purchase::Table, Purchase::Product)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Purchase::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Name,
    PhoneNumber,
    HashedPassword,
}

#[derive(Iden)]
pub enum Product {
    Table,
    Id,
    Name,
    Description,
    Seller,
    Stock,
    Price,
}

#[derive(Iden)]
pub enum Purchase {
    Table,
    Id,
    Buyer,
    Product,
    Quantity,
    UnitPrice,
    Date,
    PaidDate,
}

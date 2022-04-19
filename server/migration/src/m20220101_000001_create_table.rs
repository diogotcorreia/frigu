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
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::PhoneNumber).string().not_null())
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
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Product::Seller).integer().not_null())
                    .col(ColumnDef::new(Product::Stock).integer().not_null())
                    .col(ColumnDef::new(Product::Price).integer().not_null())
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
                    .table(Transaction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Transaction::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Transaction::Buyer).integer().not_null())
                    .col(ColumnDef::new(Transaction::Seller).integer().not_null())
                    .col(ColumnDef::new(Transaction::Quantity).integer().not_null())
                    .col(ColumnDef::new(Transaction::UnitPrice).integer().not_null())
                    .col(ColumnDef::new(Transaction::PaidDate).date_time())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-transaction-buyer")
                            .from(Transaction::Table, Transaction::Buyer)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-transaction-seller")
                            .from(Transaction::Table, Transaction::Seller)
                            .to(User::Table, User::Id)
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
            .drop_table(Table::drop().table(Transaction::Table).to_owned())
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
    Seller,
    Stock,
    Price,
}

#[derive(Iden)]
pub enum Transaction {
    Table,
    Id,
    Buyer,
    Seller,
    Quantity,
    UnitPrice,
    PaidDate,
}

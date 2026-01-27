use sea_orm_migration::{prelude::*, schema::*};

use crate::migrations::m20260126_000001_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Book::Table)
                    .if_not_exists()
                    .col(pk_auto(Book::Id))
                    .col(string(Book::Title).not_null())
                    .col(string(Book::Authors).not_null())
                    .col(text(Book::Description))
                    .col(text(Book::Comment))
                    .col(ColumnDef::new(Book::OwnerId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-book-owner_id")
                            .from(Book::Table, Book::OwnerId)
                            .to(User::Table, User::Id),
                    )
                    .col(ColumnDef::new(Book::CurrentHolderId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-book-current_holder_id")
                            .from(Book::Table, Book::CurrentHolderId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Book::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Book {
    Table,
    Id,
    Title,
    Authors,
    Description,
    Comment,
    OwnerId,
    CurrentHolderId,
}

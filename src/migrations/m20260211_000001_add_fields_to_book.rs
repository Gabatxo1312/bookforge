use sea_orm_migration::{prelude::*, schema::*};

use crate::migrations::m20260126_000002_create_book_table::Book;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Book::Table)
                    .add_column(string_null(Book::Publisher))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Book::Table)
                    .add_column(string_null(Book::PublishedDate))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Book::Table)
                    .add_column(integer_null(Book::PageCount))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Book::Table)
                    .drop_column(Book::Publisher)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Book::Table)
                    .drop_column(Book::PublishedDate)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Book::Table)
                    .drop_column(Book::PageCount)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

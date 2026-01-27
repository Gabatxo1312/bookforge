use sea_orm::ActiveValue::Set;
use sea_orm::DeleteResult;
use sea_orm::QueryOrder;
use sea_orm::entity::prelude::*;
use snafu::ResultExt;
use snafu::prelude::*;

use crate::routes::book::BookForm;
use crate::state::AppState;
use crate::state::error::BookSnafu;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "book")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub authors: String,
    pub description: Option<String>,
    pub comment: Option<String>,
    pub owner_id: i32,
    #[sea_orm(belongs_to, relation_enum = "Owner", from = "owner_id", to = "id")]
    pub owner: HasOne<super::user::Entity>,
    pub current_holder_id: Option<i32>,
    #[sea_orm(
        belongs_to,
        relation_enum = "CurrentHolder",
        from = "current_holder_id",
        to = "id"
    )]
    pub current_holder: HasOne<super::user::Entity>,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum BookError {
    // #[snafu(display("The Content Folder (Path: {path}) does not exist"))]
    // NotFound { path: String },
    #[snafu(display("Database error"))]
    DB { source: sea_orm::DbErr },
    #[snafu(display("Book with id {id} not found"))]
    NotFound { id: i32 },
}

#[derive(Debug)]
pub struct BookOperator {
    pub state: AppState,
}

impl BookOperator {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub async fn list(&self) -> Result<Vec<Model>, BookError> {
        Entity::find()
            .order_by_desc(Column::Id)
            .all(&self.state.db)
            .await
            .context(DBSnafu)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Model, BookError> {
        let book_by_id = Entity::find_by_id(id)
            .one(&self.state.db)
            .await
            .context(DBSnafu)?;

        if let Some(book) = book_by_id {
            Ok(book)
        } else {
            Err(BookError::NotFound { id })
        }
    }

    pub async fn create(&self, form: BookForm) -> Result<Model, BookError> {
        let book = ActiveModel {
            title: Set(form.title.clone()),
            authors: Set(form.authors.clone()),
            owner_id: Set(form.owner_id.clone()),
            current_holder_id: Set(form.current_holder_id.clone()),
            description: Set(form.description.clone()),
            comment: Set(form.comment.clone()),
            ..Default::default()
        };

        book.insert(&self.state.db).await.context(DBSnafu)
    }

    pub async fn update(&self, id: i32, form: BookForm) -> Result<Model, BookError> {
        let book_by_id = Self::find_by_id(&self, id).await.context(BookSnafu);

        if let Ok(book) = book_by_id {
            let mut book: ActiveModel = book.into();

            book.title = Set(form.title.clone());
            book.authors = Set(form.authors.clone());
            book.owner_id = Set(form.owner_id.clone());
            book.current_holder_id = Set(form.current_holder_id.clone());
            book.description = Set(form.description.clone());
            book.comment = Set(form.comment.clone());

            book.update(&self.state.db).await.context(DBSnafu)
        } else {
            Err(BookError::NotFound { id })
        }
    }

    pub async fn delete(&self, id: i32) -> Result<DeleteResult, BookError> {
        let book: Option<Model> = Entity::find_by_id(id)
            .one(&self.state.db)
            .await
            .context(DBSnafu)?;
        let book: Model = book.unwrap();

        book.delete(&self.state.db).await.context(DBSnafu)
    }
}

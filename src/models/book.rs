use sea_orm::ActiveValue::Set;
use sea_orm::Condition;
use sea_orm::DeleteResult;
use sea_orm::QueryOrder;
use sea_orm::entity::prelude::*;
use snafu::ResultExt;
use snafu::prelude::*;

use crate::routes::book::BookForm;
use crate::routes::book::BookQuery;
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
    /// Db Error from SeaOrm
    #[snafu(display("Database error"))]
    DB { source: sea_orm::DbErr },
    /// When Book with Id is not found
    #[snafu(display("Book with id {id} not found"))]
    NotFound { id: i32 },
}

#[derive(Debug)]
/// Operator for the CRUD on Book Model
pub struct BookOperator {
    pub state: AppState,
}

impl BookOperator {
    /// Creates a new `BookOperator` with the given application state.
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    /// Lists all books matching the optional query filters.
    ///
    /// Results are ordered by ID in descending order (newest first).
    pub async fn list(&self, query: Option<BookQuery>) -> Result<Vec<Model>, BookError> {
        let mut conditions = Condition::all();
        if let Some(book_query) = query {
            if let Some(title) = book_query.title {
                conditions = conditions.add(Column::Title.contains(&title));
            }

            if let Some(authors) = book_query.authors {
                conditions = conditions.add(Column::Authors.contains(&authors));
            }

            if let Some(owner_id) = book_query.owner_id {
                conditions = conditions.add(Column::OwnerId.eq(owner_id));
            }

            if let Some(current_holder_id) = book_query.current_holder_id {
                conditions = conditions.add(Column::CurrentHolderId.eq(current_holder_id));
            }
        }

        Entity::find()
            .filter(conditions)
            .order_by_desc(Column::Id)
            .all(&self.state.db)
            .await
            .context(DBSnafu)
    }

    /// Finds a book by its ID.
    ///
    /// # Errors
    /// Returns `BookError::NotFound` if no book exists with the given ID.
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

    /// Creates a new book from the given form data.
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

    /// Update a book (find with ID) from the given form data
    ///
    /// # Error
    /// Returns BookError::NotFound if id is not found in database
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

    /// Delete a book (find with ID)
    pub async fn delete(&self, id: i32) -> Result<DeleteResult, BookError> {
        let book: Option<Model> = Entity::find_by_id(id)
            .one(&self.state.db)
            .await
            .context(DBSnafu)?;
        let book: Model = book.unwrap();

        book.delete(&self.state.db).await.context(DBSnafu)
    }
}

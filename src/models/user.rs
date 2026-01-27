use crate::routes::user::UserForm;
use crate::state::AppState;
use sea_orm::ActiveValue::Set;
use sea_orm::DeleteResult;
use sea_orm::entity::prelude::*;
use snafu::ResultExt;
use snafu::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    // #[sea_orm(has_many, relation_enum = "Owner", from = "id", to = "owner_id")]
    // pub books: HasMany<super::book::Entity>,
    // #[sea_orm(
    //     has_many,
    //     relation_enum = "CurrentHolder",
    //     from = "id",
    //     to = "current_holder_id"
    // )]
    // pub books_borrowed: HasMany<super::book::Entity>,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum UserError {
    // #[snafu(display("The Content Folder (Path: {path}) does not exist"))]
    // NotFound { path: String },
    #[snafu(display("Database error"))]
    DB { source: sea_orm::DbErr },
    #[snafu(display("User with id {id} not found"))]
    NotFound { id: i32 },
}

#[derive(Debug)]
pub struct UserOperator {
    pub state: AppState,
}

impl UserOperator {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub async fn list(&self) -> Result<Vec<Model>, UserError> {
        Entity::find().all(&self.state.db).await.context(DBSnafu)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Model, UserError> {
        let user: Option<Model> = Entity::find_by_id(id)
            .one(&self.state.db)
            .await
            .context(DBSnafu)?;

        if let Some(user) = user {
            Ok(user)
        } else {
            Err(UserError::NotFound { id })
        }
    }

    pub async fn create(&self, form: UserForm) -> Result<Model, UserError> {
        let user = ActiveModel {
            name: Set(form.name),
            ..Default::default()
        };

        user.insert(&self.state.db).await.context(DBSnafu)
    }

    pub async fn delete(&self, user_id: i32) -> Result<DeleteResult, UserError> {
        let user: Option<Model> = Entity::find_by_id(user_id)
            .one(&self.state.db)
            .await
            .context(DBSnafu)?;
        let user: Model = user.unwrap();

        user.delete(&self.state.db).await.context(DBSnafu)
    }
}

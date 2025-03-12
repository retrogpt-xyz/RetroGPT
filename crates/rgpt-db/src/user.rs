use std::sync::Arc;

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, Selectable, SelectableHelper};

use crate::{Database, RunQueryDsl, chat, schema};

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: i32,
    pub google_id: String,
    pub email: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_login: NaiveDateTime,
}

impl User {
    pub async fn get_by_id(
        db: Arc<Database>,
        user_id: i32,
    ) -> Result<User, libserver::ServiceError> {
        let user = schema::users::table
            .find(user_id)
            .limit(1)
            .get_result(db)
            .await?;
        Ok(user)
    }

    pub async fn get_by_google_id(
        db: Arc<Database>,
        google_id: &str,
    ) -> Result<User, libserver::ServiceError> {
        let user = schema::users::table
            .filter(schema::users::google_id.eq(google_id.to_owned()))
            .get_result(db)
            .await?;
        Ok(user)
    }

    pub async fn create(
        db: Arc<Database>,
        google_id: String,
        email: String,
        name: String,
    ) -> Result<User, libserver::ServiceError> {
        NewUser {
            google_id,
            email,
            name,
        }
        .create(db)
        .await
    }

    pub async fn get_chats(
        &self,
        db: Arc<Database>,
    ) -> Result<Vec<chat::Chat>, libserver::ServiceError> {
        let chats = schema::chats::table
            .filter(schema::chats::user_id.eq(self.user_id))
            .get_results(db)
            .await?;
        Ok(chats)
    }

    pub async fn default(db: Arc<Database>) -> Result<User, libserver::ServiceError> {
        let user = schema::users::table.find(1).get_result(db).await?;
        Ok(user)
    }
}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
struct NewUser {
    pub google_id: String,
    pub email: String,
    pub name: String,
}

impl NewUser {
    async fn create(self, db: Arc<Database>) -> Result<User, libserver::ServiceError> {
        let user = diesel::insert_into(schema::users::table)
            .values(self)
            .returning(User::as_returning())
            .get_result(db)
            .await?;
        Ok(user)
    }
}

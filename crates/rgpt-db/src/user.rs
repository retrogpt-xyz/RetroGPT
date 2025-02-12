use std::{error::Error, sync::Arc};

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, Selectable, SelectableHelper};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::{chat, schema, Database};

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
    #[deprecated]
    pub async fn get_by_id(url: &str, id: i32) -> Result<User, Box<dyn Error>> {
        let conn = &mut AsyncPgConnection::establish(url).await?;

        schema::users::table
            .find(id)
            .first::<User>(conn)
            .await
            .map_err(|e| e.into())
    }

    pub async fn n_get_by_id(
        db: Arc<Database>,
        user_id: i32,
    ) -> Result<User, libserver::ServiceError> {
        let query = schema::users::table.find(user_id).limit(1);
        let user = crate::RunQueryDsl::get_result::<User>(query, db).await?;
        Ok(user)
    }

    #[deprecated]
    pub async fn get_by_google_id(url: &str, google_id: &str) -> Result<User, Box<dyn Error>> {
        let conn = &mut AsyncPgConnection::establish(url).await?;

        schema::users::table
            .filter(schema::users::google_id.eq(google_id))
            .select(User::as_select())
            .get_result(conn)
            .await
            .map_err(|e| e.into())
    }

    pub async fn n_get_by_google_id(
        db: Arc<Database>,
        google_id: &str,
    ) -> Result<User, libserver::ServiceError> {
        let query = schema::users::table.filter(schema::users::google_id.eq(google_id.to_owned()));
        let user = crate::RunQueryDsl::get_result::<User>(query, db).await?;
        Ok(user)
    }

    #[deprecated]
    pub async fn create(
        url: &str,
        google_id: String,
        email: String,
        name: String,
    ) -> Result<User, Box<dyn Error>> {
        NewUser {
            google_id,
            email,
            name,
        }
        .create(url)
        .await
    }

    pub async fn n_create(
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
        .n_create(db)
        .await
    }

    pub async fn get_chats(&self, url: &str) -> Result<Vec<chat::Chat>, Box<dyn Error>> {
        let conn = &mut AsyncPgConnection::establish(url).await?;

        schema::chats::table
            .filter(schema::chats::user_id.eq(self.user_id))
            .get_results(conn)
            .await
            .map_err(|e| e.into())
    }

    #[deprecated]
    pub async fn default(url: &str) -> Result<User, Box<dyn Error>> {
        let conn = &mut AsyncPgConnection::establish(url).await?;

        schema::users::table
            .find(1)
            .first(conn)
            .await
            .map_err(|e| e.into())
    }

    pub async fn n_default(db: Arc<Database>) -> Result<User, libserver::ServiceError> {
        let query = schema::users::table.find(1).limit(1);
        crate::RunQueryDsl::get_result::<User>(query, db)
            .await
            .map_err(|e| e.into())
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
    #[deprecated]
    async fn create(self, url: &str) -> Result<User, Box<dyn Error>> {
        let conn = &mut AsyncPgConnection::establish(url).await?;

        diesel::insert_into(schema::users::table)
            .values(self)
            .returning(User::as_returning())
            .get_result(conn)
            .await
            .map_err(|e| e.into())
    }

    async fn n_create(self, db: Arc<Database>) -> Result<User, libserver::ServiceError> {
        let query = diesel::insert_into(schema::users::table)
            .values(self)
            .returning(User::as_returning());
        let user = crate::RunQueryDsl::get_result(query, db).await?;
        Ok(user)
    }
}

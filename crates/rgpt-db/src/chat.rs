use std::{error::Error, sync::Arc};

use chrono::NaiveDateTime;
use diesel::{
    prelude::Insertable, ExpressionMethods, QueryDsl, Queryable, Selectable, SelectableHelper,
};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::{msg, schema, Database};

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::chats)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Chat {
    pub id: i32,
    pub head_msg: Option<i32>,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: Option<String>,
}

impl Chat {
    #[deprecated]
    pub async fn get_by_id(url: &str, id: i32) -> Result<Chat, Box<dyn Error>> {
        let conn = &mut AsyncPgConnection::establish(url).await?;

        schema::chats::table
            .find(id)
            .first::<Chat>(conn)
            .await
            .map_err(|e| e.into())
    }

    pub async fn n_get_by_id(db: Arc<Database>, id: i32) -> Result<Chat, libserver::ServiceError> {
        let query = schema::chats::table.find(id);
        let chat = crate::RunQueryDsl::get_result::<Chat>(query, db).await?;
        Ok(chat)
    }

    #[deprecated]
    pub async fn append_to_chat(&self, url: &str, msg: msg::Msg) -> Result<Chat, Box<dyn Error>> {
        let conn = &mut AsyncPgConnection::establish(url).await?;

        diesel::update(schema::chats::table.find(self.id))
            .set((
                schema::chats::head_msg.eq(msg.id),
                schema::chats::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .returning(Chat::as_returning())
            .get_result(conn)
            .await
            .map_err(|e| e.into())
    }

    pub async fn n_append_to_chat(
        &self,
        db: Arc<Database>,
        msg: &msg::Msg,
    ) -> Result<Chat, libserver::ServiceError> {
        let query = diesel::update(schema::chats::table.find(self.id))
            .set((
                schema::chats::head_msg.eq(msg.id),
                schema::chats::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .returning(Chat::as_returning());
        let chat = crate::RunQueryDsl::get_result(query, db).await?;
        Ok(chat)
    }

    #[deprecated]
    pub async fn create(
        url: &str,
        user_id: i32,
        name: Option<String>,
    ) -> Result<Chat, Box<dyn Error>> {
        NewChat { user_id, name }.create(url).await
    }

    pub async fn n_create(
        db: Arc<Database>,
        user_id: i32,
        name: Option<String>,
    ) -> Result<Chat, libserver::ServiceError> {
        NewChat { user_id, name }.n_create(db).await
    }
}

#[derive(Insertable)]
#[diesel(table_name = schema::chats)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct NewChat {
    pub user_id: i32,
    pub name: Option<String>,
}

impl NewChat {
    #[deprecated]
    async fn create(self, url: &str) -> Result<Chat, Box<dyn Error>> {
        let conn = &mut AsyncPgConnection::establish(url).await?;

        diesel::insert_into(schema::chats::table)
            .values(self)
            .returning(Chat::as_returning())
            .get_result(conn)
            .await
            .map_err(|e| e.into())
    }

    async fn n_create(self, db: Arc<Database>) -> Result<Chat, libserver::ServiceError> {
        let query = diesel::insert_into(schema::chats::table)
            .values(self)
            .returning(Chat::as_returning());
        let chat = crate::RunQueryDsl::get_result(query, db).await?;
        Ok(chat)
    }
}

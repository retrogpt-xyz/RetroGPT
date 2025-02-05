use std::error::Error;

use chrono::NaiveDateTime;
use diesel::{
    prelude::Insertable, ExpressionMethods, QueryDsl, Queryable, Selectable, SelectableHelper,
};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::{msg, schema};

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::chats)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Chat {
    pub id: i32,
    pub head_msg: Option<i32>,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
}

impl Chat {
    pub async fn get_by_id(url: &str, id: i32) -> Result<Chat, Box<dyn Error>> {
        let conn = &mut AsyncPgConnection::establish(url).await?;

        schema::chats::table
            .find(id)
            .first::<Chat>(conn)
            .await
            .map_err(|e| e.into())
    }

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

    pub async fn create(url: &str, user_id: i32, name: String) -> Result<Chat, Box<dyn Error>> {
        NewChat { user_id, name }.create(url).await
    }
}

#[derive(Insertable)]
#[diesel(table_name = schema::chats)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct NewChat {
    pub user_id: i32,
    pub name: String,
}

impl NewChat {
    async fn create(self, url: &str) -> Result<Chat, Box<dyn Error>> {
        let conn = &mut AsyncPgConnection::establish(url).await?;

        diesel::insert_into(schema::chats::table)
            .values(self)
            .returning(Chat::as_returning())
            .get_result(conn)
            .await
            .map_err(|e| e.into())
    }
}

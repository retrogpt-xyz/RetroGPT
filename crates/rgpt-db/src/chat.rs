use std::error::Error;

use diesel::{prelude::Insertable, QueryDsl, Queryable, Selectable, SelectableHelper};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::schema;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::chats)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Chat {
    pub id: i32,
    pub head_msg: Option<i32>,
    pub user_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub name: String,
}

impl Chat {
    pub async fn get_by_id(url: &str, id: i32) -> Result<Chat, Box<dyn Error>> {
        let conn = &mut AsyncPgConnection::establish(url).await?;

        Ok(schema::chats::table.find(id).first::<Chat>(conn).await?)
    }

    pub async fn append_to_chat(url: &str, chat: Chat, msg: ()) {
        todo!()
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

        Ok(diesel::insert_into(schema::chats::table)
            .values(self)
            .returning(Chat::as_returning())
            .get_result(conn)
            .await?)
    }
}

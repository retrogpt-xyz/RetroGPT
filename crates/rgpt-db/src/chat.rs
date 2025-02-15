use std::sync::Arc;

use chrono::NaiveDateTime;
use diesel::{
    prelude::Insertable, ExpressionMethods, QueryDsl, Queryable, Selectable, SelectableHelper,
};

use crate::{msg, schema, Database, RunQueryDsl};

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
    pub async fn n_get_by_id(db: Arc<Database>, id: i32) -> Result<Chat, libserver::ServiceError> {
        let chat = schema::chats::table.find(id).get_result(db).await?;
        Ok(chat)
    }

    pub async fn n_append_to_chat(
        &self,
        db: Arc<Database>,
        msg: &msg::Msg,
    ) -> Result<Chat, libserver::ServiceError> {
        let chat = diesel::update(schema::chats::table.find(self.id))
            .set((
                schema::chats::head_msg.eq(msg.id),
                schema::chats::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .returning(Chat::as_returning())
            .get_result(db)
            .await?;
        Ok(chat)
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
    async fn n_create(self, db: Arc<Database>) -> Result<Chat, libserver::ServiceError> {
        let chat = diesel::insert_into(schema::chats::table)
            .values(self)
            .returning(Chat::as_returning())
            .get_result(db)
            .await?;
        Ok(chat)
    }
}

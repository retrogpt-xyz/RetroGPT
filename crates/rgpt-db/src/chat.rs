use std::sync::Arc;

use chrono::NaiveDateTime;
use diesel::{
    ExpressionMethods, QueryDsl, Queryable, Selectable, SelectableHelper, prelude::Insertable,
};

use crate::{
    Database, RunQueryDsl,
    msg::{self, Msg},
    schema,
};

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
    pub deleted: bool,
}

impl Chat {
    pub async fn get_by_id(db: Arc<Database>, id: i32) -> Result<Chat, libserver::ServiceError> {
        let chat = schema::chats::table.find(id).get_result(db).await?;
        Ok(chat)
    }

    pub async fn append_to_chat(
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

    pub async fn create(
        db: Arc<Database>,
        user_id: i32,
        name: Option<String>,
    ) -> Result<Chat, libserver::ServiceError> {
        NewChat {
            user_id,
            name,
            deleted: false,
        }
        .create(db)
        .await
    }

    pub async fn msg_chain(&self, db: Arc<Database>) -> Result<Vec<Msg>, libserver::ServiceError> {
        let msgs = match self.head_msg {
            Some(msg_id) => {
                Msg::get_by_id(db.clone(), msg_id)
                    .await?
                    .get_msg_chain(db.clone())
                    .await?
            }
            None => vec![],
        };

        Ok(msgs)
    }

    pub async fn delete(mut self, db: Arc<Database>) -> Result<(), libserver::ServiceError> {
        self.deleted = true;
        diesel::update(schema::chats::table.find(self.id))
            .set(schema::chats::deleted.eq(true))
            .execute(db)
            .await?;
        Ok(())
    }
}

#[derive(Insertable)]
#[diesel(table_name = schema::chats)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct NewChat {
    pub user_id: i32,
    pub name: Option<String>,
    pub deleted: bool,
}

impl NewChat {
    async fn create(self, db: Arc<Database>) -> Result<Chat, libserver::ServiceError> {
        let chat = diesel::insert_into(schema::chats::table)
            .values(self)
            .returning(Chat::as_returning())
            .get_result(db)
            .await?;
        Ok(chat)
    }
}

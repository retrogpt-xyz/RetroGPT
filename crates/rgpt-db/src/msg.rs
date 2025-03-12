use std::sync::Arc;

use chrono::NaiveDateTime;
use diesel::{QueryDsl, Queryable, Selectable, SelectableHelper, prelude::Insertable};

use crate::{Database, RunQueryDsl, schema};

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = schema::msgs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Msg {
    pub id: i32,
    pub body: String,
    pub sender: String,
    pub user_id: i32,
    pub parent_message_id: Option<i32>,
    pub created_at: NaiveDateTime,
}

impl Msg {
    pub async fn get_by_id(db: Arc<Database>, id: i32) -> Result<Msg, libserver::ServiceError> {
        let msg = schema::msgs::table.find(id).get_result::<Msg>(db).await?;
        Ok(msg)
    }

    pub async fn create(
        db: Arc<Database>,
        body: String,
        sender: String,
        user_id: i32,
        parent_message_id: Option<i32>,
    ) -> Result<Msg, libserver::ServiceError> {
        NewMsg {
            body,
            sender,
            user_id,
            parent_message_id,
        }
        .create(db)
        .await
    }

    pub async fn get_msg_chain(
        self,
        db: Arc<Database>,
    ) -> Result<Vec<Msg>, libserver::ServiceError> {
        match self.parent_message_id {
            Some(id) => {
                let parent = Msg::get_by_id(db.clone(), id).await?;
                let mut parents = Box::pin(parent.get_msg_chain(db.clone())).await?;
                parents.push(self);
                Ok(parents)
            }
            None => Ok(vec![self]),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = schema::msgs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct NewMsg {
    pub body: String,
    pub sender: String,
    pub user_id: i32,
    pub parent_message_id: Option<i32>,
}

impl NewMsg {
    async fn create(self, db: Arc<Database>) -> Result<Msg, libserver::ServiceError> {
        let msg = diesel::insert_into(schema::msgs::table)
            .values(self)
            .returning(Msg::as_returning())
            .get_result(db)
            .await?;
        Ok(msg)
    }
}

use std::error::Error;

use chrono::NaiveDateTime;
use diesel::{prelude::Insertable, QueryDsl, Queryable, Selectable, SelectableHelper};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::schema;

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
    pub async fn get_by_id(url: &str, id: i32) -> Result<Msg, Box<dyn Error>> {
        let conn = &mut AsyncPgConnection::establish(url).await?;

        schema::msgs::table
            .find(id)
            .first::<Msg>(conn)
            .await
            .map_err(|e| e.into())
    }

    pub async fn create(
        url: &str,
        body: String,
        sender: String,
        user_id: i32,
        parent_message_id: Option<i32>,
    ) -> Result<Msg, Box<dyn Error>> {
        NewMsg {
            body,
            sender,
            user_id,
            parent_message_id,
        }
        .create(url)
        .await
    }

    pub async fn get_msg_chain(self, url: &str) -> Result<Vec<Msg>, Box<dyn Error>> {
        match self.parent_message_id {
            Some(id) => {
                let parent = Msg::get_by_id(url, id).await?;
                let mut parents = Box::pin(parent.get_msg_chain(url)).await?;
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
    async fn create(&self, url: &str) -> Result<Msg, Box<dyn Error>> {
        let conn = &mut AsyncPgConnection::establish(url).await?;

        diesel::insert_into(schema::msgs::table)
            .values(self)
            .returning(Msg::as_returning())
            .get_result(conn)
            .await
            .map_err(|e| e.into())
    }
}

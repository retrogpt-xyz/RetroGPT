use std::{error::Error, sync::Arc};

use chrono::NaiveDateTime;
use diesel::{prelude::Insertable, QueryDsl, Queryable, Selectable, SelectableHelper};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::{schema, Database};

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
    #[deprecated]
    pub async fn get_by_id(url: &str, id: i32) -> Result<Msg, Box<dyn Error>> {
        let conn = &mut AsyncPgConnection::establish(url).await?;

        schema::msgs::table
            .find(id)
            .first::<Msg>(conn)
            .await
            .map_err(|e| e.into())
    }

    pub async fn n_get_by_id(db: Arc<Database>, id: i32) -> Result<Msg, libserver::ServiceError> {
        let query = schema::msgs::table.find(id);
        let msg = crate::RunQueryDsl::get_result::<Msg>(query, db).await?;
        Ok(msg)
    }

    #[deprecated]
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

    pub async fn n_create(
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
        .n_create(db)
        .await
    }

    #[deprecated]
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

    pub async fn n_get_msg_chain(
        self,
        db: Arc<Database>,
    ) -> Result<Vec<Msg>, libserver::ServiceError> {
        match self.parent_message_id {
            Some(id) => {
                let parent = Msg::n_get_by_id(db.clone(), id).await?;
                let mut parents = Box::pin(parent.n_get_msg_chain(db.clone())).await?;
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

    async fn n_create(self, db: Arc<Database>) -> Result<Msg, libserver::ServiceError> {
        let query = diesel::insert_into(schema::msgs::table)
            .values(self)
            .returning(Msg::as_returning());
        let msg = crate::RunQueryDsl::get_result(query, db).await?;
        Ok(msg)
    }
}

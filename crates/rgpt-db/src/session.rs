use std::error::Error;

use chrono::{Duration, NaiveDateTime, Utc};
use diesel::{
    prelude::Insertable, ExpressionMethods, QueryDsl, Queryable, Selectable, SelectableHelper,
};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::{schema, user};

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
    pub session_token: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

impl Session {
    pub async fn get_by_token(url: &str, token: String) -> Result<Session, Box<dyn Error>> {
        let conn = &mut AsyncPgConnection::establish(url).await?;

        schema::sessions::table
            .find(token)
            .first(conn)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete(self, url: &str) -> Result<(), Box<dyn Error>> {
        let conn = &mut AsyncPgConnection::establish(url).await?;

        diesel::delete(schema::sessions::table.find(self.session_token))
            .execute(conn)
            .await
            .map(|_| ())
            .map_err(|e| e.into())
    }

    pub fn validate(&self) -> bool {
        expires_at_is_valid(&self.expires_at)
    }

    pub async fn create(url: &str, user_id: i32) -> Result<Session, Box<dyn Error>> {
        let expires_at: NaiveDateTime = Utc::now().naive_utc() + Duration::hours(1);
        let session_token = uuid::Uuid::new_v4().into();

        NewSession {
            user_id,
            expires_at,
            session_token,
        }
        .create(url)
        .await
    }

    pub async fn get_session_for_user(
        url: &str,
        user: user::User,
    ) -> Result<Session, Box<dyn Error>> {
        let conn = &mut AsyncPgConnection::establish(url).await?;

        match schema::sessions::table
            .filter(schema::sessions::user_id.eq(user.user_id))
            .first::<Session>(conn)
            .await
        {
            Ok(session) => {
                if session.validate() {
                    return Ok(session);
                }
            }
            Err(_) => {}
        };

        Self::create(url, user.user_id).await.map_err(|e| e.into())
    }
}

#[derive(Insertable)]
#[diesel(table_name = schema::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct NewSession {
    pub session_token: String,
    pub user_id: i32,
    pub expires_at: NaiveDateTime,
}

impl NewSession {
    async fn create(self, url: &str) -> Result<Session, Box<dyn Error>> {
        let conn = &mut AsyncPgConnection::establish(url).await?;

        diesel::insert_into(schema::sessions::table)
            .values(self)
            .returning(Session::as_returning())
            .get_result(conn)
            .await
            .map_err(|e| e.into())
    }
}

pub fn expires_at_is_valid(expires_at: &NaiveDateTime) -> bool {
    expires_at > &Utc::now().naive_utc()
}

use std::sync::Arc;

use chrono::{Duration, NaiveDateTime, Utc};
use diesel::{
    ExpressionMethods, QueryDsl, Queryable, Selectable, SelectableHelper, prelude::Insertable,
};

use crate::{Database, RunQueryDsl, schema, user::User};

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
    pub async fn get_by_token(
        db: Arc<Database>,
        token: String,
    ) -> Result<Session, libserver::ServiceError> {
        let session = schema::sessions::table
            .find(token)
            .get_result::<Session>(db)
            .await?;
        Ok(session)
    }

    pub async fn delete(self, db: Arc<Database>) -> Result<(), libserver::ServiceError> {
        diesel::delete(schema::sessions::table.find(self.session_token))
            .execute(db)
            .await?;
        Ok(())
    }

    pub fn validate(&self) -> bool {
        expires_at_is_valid(&self.expires_at)
    }

    pub async fn create(
        db: Arc<Database>,
        user_id: i32,
    ) -> Result<Session, libserver::ServiceError> {
        let expires_at: NaiveDateTime = Utc::now().naive_utc() + Duration::hours(1);
        let session_token = uuid::Uuid::new_v4().into();

        NewSession {
            user_id,
            expires_at,
            session_token,
        }
        .create(db)
        .await
    }

    pub async fn get_for_user(
        db: Arc<Database>,
        user: &User,
    ) -> Result<Session, libserver::ServiceError> {
        let existing_session = schema::sessions::table
            .filter(schema::sessions::user_id.eq(user.user_id))
            .limit(1)
            .get_result::<Session>(db.clone())
            .await;

        if let Ok(session) = existing_session {
            if session.validate() {
                return Ok(session);
            } else {
                session.delete(db.clone()).await?
            }
        }

        Session::create(db, user.user_id).await
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
    pub async fn create(self, db: Arc<Database>) -> Result<Session, libserver::ServiceError> {
        let session = diesel::insert_into(schema::sessions::table)
            .values(self)
            .returning(Session::as_returning())
            .get_result(db)
            .await?;
        Ok(session)
    }
}

pub fn expires_at_is_valid(expires_at: &NaiveDateTime) -> bool {
    expires_at > &Utc::now().naive_utc()
}

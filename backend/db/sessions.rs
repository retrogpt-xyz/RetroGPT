use diesel::{
    prelude::Insertable, ExpressionMethods, QueryDsl, Queryable, Selectable, SelectableHelper,
};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::schema::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
    pub session_token: String,
    pub user_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub expires_at: chrono::NaiveDateTime,
}

pub async fn get_session(conn: &mut AsyncPgConnection, user: &super::users::User) -> Session {
    use super::schema::sessions::dsl::*;

    match sessions
        .filter(user_id.eq(user.user_id))
        .select(Session::as_select())
        .get_result(conn)
        .await
        .ok()
    {
        Some(sess) => {
            if expires_at_is_valid(&sess.expires_at) {
                return sess;
            } else {
                delete_session_by_token(conn, &sess.session_token).await;
            }
        }
        None => (),
    };

    diesel::insert_into(sessions)
        .values(Session {
            session_token: uuid::Uuid::new_v4().into(),
            user_id: user.user_id,
            created_at: chrono::Utc::now().naive_utc(),
            expires_at: (chrono::Utc::now() + chrono::Duration::hours(1)).naive_utc(),
        })
        .returning(Session::as_returning())
        .get_result(conn)
        .await
        .unwrap()
}

pub async fn session_token_is_valid(conn: &mut AsyncPgConnection, sess_token: &str) -> bool {
    use super::schema::sessions::dsl::*;

    let results: Option<Session> = sessions
        .filter(session_token.eq(sess_token.to_string()))
        .select(Session::as_select())
        .get_result(conn)
        .await
        .ok();

    match results {
        Some(sess) => {
            if expires_at_is_valid(&sess.expires_at) {
                return true;
            } else {
                delete_session_by_token(conn, &sess.session_token).await;
                return false;
            }
        }
        None => return false,
    }
}

async fn delete_session_by_token(conn: &mut AsyncPgConnection, sess_token: &str) {
    use super::schema::sessions::dsl::*;

    diesel::delete(sessions.filter(session_token.eq(sess_token)))
        .execute(conn)
        .await
        .unwrap();
}

fn expires_at_is_valid(expires_at: &chrono::NaiveDateTime) -> bool {
    expires_at > &chrono::Utc::now().naive_utc()
}

pub async fn get_session_by_token(
    conn: &mut AsyncPgConnection,
    sess_token: &str,
) -> Option<Session> {
    use super::schema::sessions::dsl::*;

    let result: Option<Session> = sessions
        .filter(session_token.eq(sess_token))
        .select(Session::as_select())
        .get_result(conn)
        .await
        .ok();

    match result {
        Some(sess) => {
            if expires_at_is_valid(&sess.expires_at) {
                Some(sess)
            } else {
                delete_session_by_token(conn, &sess.session_token).await;
                None
            }
        }
        None => None,
    }
}

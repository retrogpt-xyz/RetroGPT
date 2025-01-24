use diesel::{QueryDsl, Queryable, Selectable, SelectableHelper, TextExpressionMethods};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: i32,
    pub google_id: String,
    pub email: String,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub last_login: chrono::NaiveDateTime,
}

pub async fn get_default_user(conn: &mut AsyncPgConnection) -> User {
    use super::schema::users::dsl::*;
    users
        .filter(name.like("Default"))
        .select(User::as_select())
        .get_result(conn)
        .await
        .unwrap()
}

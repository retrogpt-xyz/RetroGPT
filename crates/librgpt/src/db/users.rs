use super::schema::users;
use diesel::{
    ExpressionMethods, Insertable, QueryDsl, Queryable, Selectable, SelectableHelper,
    TextExpressionMethods,
};
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

#[derive(Insertable)]
#[diesel(table_name = super::schema::users)]
struct NewUser {
    pub google_id: String,
    pub email: String,
    pub name: String,
}

pub async fn get_default_user(conn: &mut AsyncPgConnection) -> User {
    use super::schema::users::dsl::*;
    users
        .filter(name.like("Default User"))
        .select(User::as_select())
        .get_result(conn)
        .await
        .unwrap()
}

pub async fn insert_user(
    conn: &mut AsyncPgConnection,
    google_id: String,
    email: String,
    name: String,
) -> User {
    use diesel::insert_into;

    let new_user = NewUser {
        google_id,
        email,
        name,
    };

    insert_into(users::table)
        .values(&new_user)
        .returning(User::as_select())
        .get_result(conn)
        .await
        .unwrap()
}

pub async fn get_user_by_id(conn: &mut AsyncPgConnection, id: i32) -> User {
    use super::schema::users::dsl::*;
    users
        .find(id)
        .select(User::as_select())
        .get_result(conn)
        .await
        .unwrap()
}

pub async fn get_user_by_google_id(
    conn: &mut AsyncPgConnection,
    google_id_value: String,
) -> Option<User> {
    super::schema::users::table
        .filter(super::schema::users::dsl::google_id.eq(google_id_value))
        .select(User::as_select())
        .get_result(conn)
        .await
        .ok()
}

pub async fn get_users_chats(conn: &mut AsyncPgConnection, user: &User) -> Vec<super::dep_chats::Chat> {
    use super::schema::chats::dsl::*;

    chats
        .filter(user_id.eq(user.user_id))
        .select(super::dep_chats::Chat::as_select())
        .load(conn)
        .await
        .unwrap()
}

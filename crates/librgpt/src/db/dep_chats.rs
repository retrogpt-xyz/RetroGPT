use diesel::{
    prelude::Insertable, ExpressionMethods, QueryDsl, Queryable, Selectable, SelectableHelper,
};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

#[derive(Queryable, Selectable)]
#[diesel(table_name = rgpt_db::schema::chats)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Chat {
    pub id: i32,
    pub head_msg: Option<i32>,
    pub user_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = rgpt_db::schema::chats)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct NewChat {
    pub head_msg: i32,
    pub user_id: i32,
    pub name: String,
}

pub async fn get_chat_by_id(conn: &mut AsyncPgConnection, chat_id: i32) -> Chat {
    use rgpt_db::schema::chats::dsl::*;

    chats
        .find(chat_id)
        .select(Chat::as_select())
        .get_result(conn)
        .await
        .unwrap()
}

pub async fn create_chat(conn: &mut AsyncPgConnection, msg: &super::msgs::Msg) -> Chat {
    let chat = NewChat {
        head_msg: msg.id,
        user_id: msg.user_id,
        name: "Untitled Chat".to_string(),
    };

    use rgpt_db::schema::chats::dsl::*;

    diesel::insert_into(chats)
        .values(chat)
        .returning(Chat::as_returning())
        .get_result(conn)
        .await
        .unwrap()
}

pub async fn add_to_chat(
    conn: &mut AsyncPgConnection,
    chat: &Chat,
    msg: &super::msgs::Msg,
) -> Chat {
    use rgpt_db::schema::chats::dsl::*;

    diesel::update(chats.find(chat.id))
        .set((
            head_msg.eq(msg.id),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .returning(Chat::as_returning())
        .get_result(conn)
        .await
        .unwrap()
}

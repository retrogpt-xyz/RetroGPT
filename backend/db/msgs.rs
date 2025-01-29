use diesel::{
    prelude::Insertable, ExpressionMethods, QueryDsl, Queryable, Selectable, SelectableHelper,
};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use tokio::sync::oneshot;

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::msgs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Msg {
    pub id: i32,
    pub body: String,
    pub sender: String,
    pub user_id: i32,
    pub parent_message_id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::msgs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct NewMsg {
    pub body: String,
    pub sender: String,
    pub user_id: i32,
    pub parent_message_id: Option<i32>,
}

pub async fn get_msg_by_id(conn: &mut AsyncPgConnection, query_id: i32) -> Msg {
    use super::schema::msgs::dsl::*;

    msgs.filter(id.eq(query_id))
        .select(Msg::as_select())
        .get_result(conn)
        .await
        .unwrap()
}

pub async fn get_all_parents(conn: &mut AsyncPgConnection, msg: Msg) -> Vec<Msg> {
    match msg.parent_message_id {
        Some(prnt_id) => {
            let prnt = get_msg_by_id(conn, prnt_id).await;
            let mut prnts = Box::pin(get_all_parents(conn, prnt)).await;
            prnts.push(msg);
            prnts
        }
        None => vec![msg],
    }
}

pub async fn create_msg(
    conn: &mut AsyncPgConnection,
    text: &str,
    msg_sender: &str,
    uid: i32,
    prnt_id: Option<i32>,
) -> Msg {
    let msg = NewMsg {
        body: text.to_string(),
        sender: msg_sender.to_string(),
        user_id: uid,
        parent_message_id: prnt_id,
    };

    use super::schema::msgs::dsl::*;

    diesel::insert_into(msgs)
        .values(msg)
        .returning(Msg::as_returning())
        .get_result(conn)
        .await
        .unwrap()
}

pub async fn create_placeholder_msg(
    conn: &mut AsyncPgConnection,
    msg_sender: &str,
    uid: i32,
    prnt_id: Option<i32>,
) -> (oneshot::Sender<String>, Msg) {
    // Create a placeholder message
    let placeholder_msg = create_msg(conn, "Placeholder message", msg_sender, uid, prnt_id).await;

    // Create a oneshot channel
    let (tx, rx) = oneshot::channel();

    // Spawn a future to wait on the receiver and update the message
    tokio::spawn(async move {
        // Establish a new connection here
        let mut new_conn = super::make_conn().await; // Replace with your connection setup logic

        if let Ok(new_body) = rx.await {
            // Update the message with the new body
            diesel::update(super::schema::msgs::dsl::msgs.find(placeholder_msg.id))
                .set(super::schema::msgs::dsl::body.eq(new_body))
                .execute(&mut new_conn) // Use the new connection
                .await
                .unwrap();
        }
    });

    (tx, placeholder_msg)
}

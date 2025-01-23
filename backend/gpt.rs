use std::{borrow::Cow, error::Error};

use serde::Deserialize;
use serde_json::{Map, Value};

use crate::{
    cfg::Cfg,
    db::{
        chats::{add_to_chat, create_chat, get_chat_by_id},
        msgs::create_msg,
    },
};

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct BackendQueryMsg<'a> {
    text: Cow<'a, str>,
    chatId: Option<i32>,
}

pub async fn gpt_api(cfg: &Cfg, be_query_msg: &str) -> Result<(String, i32), Box<dyn Error>> {
    let (api_fmted_msgs, new_head_id) = create_messages(cfg, be_query_msg).await;
    let body = serde_json::json!({
        "model": cfg.model_name,
        "messages": api_fmted_msgs,
        "max_tokens": cfg.max_tokens,
    });

    let resp = cfg
        .client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", cfg.api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    Ok((resp.text().await?, new_head_id))
}

async fn create_messages(cfg: &Cfg, backend_query_msg: &str) -> (Value, i32) {
    let backend_query_msg: BackendQueryMsg<'_> = serde_json::from_str(backend_query_msg).unwrap();

    let mut conn = cfg.db_conn.lock().await;
    let def_user = crate::db::users::get_default_user(&mut conn).await;
    let (chat, msg) = match backend_query_msg.chatId {
        Some(id) => {
            let chat = get_chat_by_id(&mut conn, id).await;
            let msg = create_msg(
                &mut conn,
                &backend_query_msg.text,
                "user",
                def_user.user_id,
                Some(chat.head_msg),
            )
            .await;
            let chat = add_to_chat(&mut conn, &chat, &msg).await;
            (chat, msg)
        }
        None => {
            let msg = create_msg(
                &mut conn,
                &backend_query_msg.text,
                "user",
                def_user.user_id,
                None,
            )
            .await;
            let chat = create_chat(&mut conn, &msg).await;
            (chat, msg)
        }
    };

    let new_head_id = chat.head_msg;

    let msg_chain = crate::db::msgs::get_all_parents(&mut conn, msg).await;

    let mut api_fmted_msgs = vec![create_message("system", &cfg.system_message)];

    api_fmted_msgs.extend(
        msg_chain
            .into_iter()
            .filter_map(|msg| match msg.sender.as_ref() {
                "user" => Some(create_message("user", msg.body)),
                "ai" => Some(create_message("assistant", msg.body)),
                _ => None,
            }),
    );

    println!("{}", Value::Array(api_fmted_msgs.clone()));
    (Value::Array(api_fmted_msgs), new_head_id)
}

fn create_message(from: impl Into<String>, msg: impl Into<String>) -> Value {
    let mut map = Map::new();

    map.insert("role".to_string(), Value::String(from.into()));
    map.insert("content".to_string(), Value::String(msg.into()));

    Value::Object(map)
}

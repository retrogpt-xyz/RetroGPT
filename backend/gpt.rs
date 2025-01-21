use std::{borrow::Cow, error::Error};

use serde::Deserialize;
use serde_json::{Map, Value};

use crate::cfg::Cfg;

#[derive(Deserialize, Debug)]
struct DisplayMsg<'a> {
    sender: Cow<'a, str>,
    text: Cow<'a, str>,
}

pub async fn gpt_api(cfg: &Cfg, chat_context: &str) -> Result<String, Box<dyn Error>> {
    let body = serde_json::json!({
        "model": cfg.model_name,
        "messages": create_messages(cfg, chat_context),
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

    Ok(resp.text().await?)
}

fn create_messages(cfg: &Cfg, chat_context: &str) -> Value {
    let msg_context: Vec<DisplayMsg<'_>> = serde_json::from_str(chat_context).unwrap();

    let mut msgs = vec![create_message("system", &cfg.system_message)];

    msgs.extend(msg_context.into_iter().filter_map(|msg| {
        println!("{msg:?}");
        match msg.sender.as_ref() {
            "user" => Some(create_message("user", msg.text)),
            "ai" => Some(create_message("assistant", msg.sender)),
            _ => None,
        }
    }));

    Value::Array(msgs)
}

fn create_message(from: impl Into<String>, msg: impl Into<String>) -> Value {
    let mut map = Map::new();

    map.insert("role".to_string(), Value::String(from.into()));
    map.insert("content".to_string(), Value::String(msg.into()));

    Value::Object(map)
}

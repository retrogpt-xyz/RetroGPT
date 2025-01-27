use std::{borrow::Cow, error::Error};

use serde::Deserialize;
use serde_json::{Map, Value};

use crate::{cfg::Cfg, db::msgs::Msg};

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct BackendQueryMsg<'a> {
    pub text: Cow<'a, str>,
    pub chatId: Option<i32>,
}

pub async fn gpt_api(cfg: &Cfg, msg_chain: Vec<Msg>) -> Result<String, Box<dyn Error>> {
    let api_fmted_msgs = create_messages(cfg, msg_chain).await;
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

    Ok(resp.text().await?)
}

async fn create_messages(cfg: &Cfg, msg_chain: Vec<Msg>) -> Value {
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
    Value::Array(api_fmted_msgs)
}

fn create_message(from: impl Into<String>, msg: impl Into<String>) -> Value {
    let mut map = Map::new();

    map.insert("role".to_string(), Value::String(from.into()));
    map.insert("content".to_string(), Value::String(msg.into()));

    Value::Object(map)
}

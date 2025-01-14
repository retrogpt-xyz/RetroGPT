use std::error::Error;

use serde_json::{Map, Value};

use crate::cfg::Cfg;

pub async fn gpt_api(cfg: &Cfg, prompt: &str) -> Result<String, Box<dyn Error>> {
    let body = serde_json::json!({
        "model": cfg.model_name,
        "messages": create_messages(cfg, prompt),
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

fn create_messages(cfg: &Cfg, prompt: &str) -> Value {
    let msgs = vec![
        create_message("system", &cfg.system_message),
        create_message("user", prompt),
    ];
    Value::Array(msgs)
}

fn create_message(user: impl Into<String>, msg: impl Into<String>) -> Value {
    let mut map = Map::new();

    map.insert("role".to_string(), Value::String(user.into()));
    map.insert("content".to_string(), Value::String(msg.into()));

    Value::Object(map)
}

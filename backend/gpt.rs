use std::error::Error;

use serde_json::{Map, Value};

use crate::cfg::Cfg;

pub async fn query_gpt(cfg: &Cfg, prompt: &str) -> Result<String, Box<dyn Error>> {
    let mut map = Map::new();

    insert_model(&mut map, cfg);
    insert_messages(&mut map, cfg, prompt);
    insert_max_tokens(&mut map, cfg);

    let body = Value::Object(map);
    let _body: serde_json::Value = serde_json::json!({
    "model": "gpt-4o-mini",
    "messages": {"role": "user", "content": prompt},
    "max_tokens": cfg.max_tokens,
    });

    let res = cfg
        .client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", cfg.api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;
    Ok(res.text().await?)
}

fn insert_model(map: &mut Map<String, Value>, cfg: &Cfg) {
    map.insert(
        "model".to_string(),
        Value::String(String::from(&cfg.model_name)),
    );
}

fn insert_max_tokens(map: &mut Map<String, Value>, cfg: &Cfg) {
    map.insert(
        "max_tokens".to_string(),
        Value::Number(cfg.max_tokens.into()),
    );
}

fn insert_messages(map: &mut Map<String, Value>, cfg: &Cfg, prompt: &str) {
    map.insert("messages".to_string(), create_messages(cfg, prompt));
}

fn create_messages(_cfg: &Cfg, prompt: &str) -> Value {
    let mut msgs = Vec::new();
    msgs.push(create_user_message(prompt));
    Value::Array(msgs)
}

fn create_user_message(prompt: &str) -> Value {
    let mut map = Map::new();

    map.insert("role".to_string(), Value::String("user".into()));
    map.insert("content".to_string(), Value::String(prompt.into()));

    Value::Object(map)
}

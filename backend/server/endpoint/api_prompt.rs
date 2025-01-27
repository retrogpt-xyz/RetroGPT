use std::convert::identity;

use http_body_util::BodyExt;
use hyper::{
    body::{Body, Bytes, Frame},
    header::CONTENT_TYPE,
    Response, StatusCode,
};
use serde_json::json;

use crate::{
    cfg::Cfg,
    db::chats::{add_to_chat, get_chat_by_id},
    gpt::gpt_api,
    server::{
        error::{error_400, error_500},
        IncReqst, OutResp,
    },
};

pub async fn api_prompt(cfg: &Cfg, req: IncReqst) -> OutResp {
    api_prompt_inner(cfg, req).await.unwrap_or_else(identity)
}

pub async fn api_prompt_inner(cfg: &Cfg, req: IncReqst) -> Result<OutResp, OutResp> {
    if req.body().size_hint().upper().unwrap_or(u64::MAX) > cfg.max_req_size {
        return Err(error_400("request body is too large"));
    }

    let bytes = req
        .collect()
        .await
        .map_err(|_| error_500())?
        .to_bytes()
        .to_vec();
    let prompt = std::str::from_utf8(&bytes).map_err(|_| error_500())?;

    // let ocfg = async_openai::config::OpenAIConfig::new().with_api_key(&cfg.api_key);
    // let client = async_openai::Client::with_config(ocfg);

    let (recvd, new_head_id, chat_id) = gpt_api(cfg, prompt).await.map_err(|_| error_500())?;
    let parsed = serde_json::from_str::<serde_json::Value>(&recvd).map_err(|_| error_500())?;

    let resp = match parsed
        .get("choices")
        .ok_or_else(error_500)?
        .get(0)
        .ok_or_else(error_500)?
        .get("message")
        .ok_or_else(error_500)?
        .get("content")
        .ok_or_else(error_500)?
    {
        serde_json::Value::String(string) => String::from(string),
        _ => Err(error_500())?,
    };

    let mut conn = cfg.db_conn.lock().await;
    let def_user = crate::db::users::get_default_user(&mut conn).await;
    let msg =
        crate::db::msgs::create_msg(&mut conn, &resp, "ai", def_user.user_id, Some(new_head_id))
            .await;
    let chat = get_chat_by_id(&mut conn, chat_id).await;
    let _ = add_to_chat(&mut conn, &chat, &msg).await;

    let be_req_msg = json!({
        "text": resp,
        "chatId": chat_id,
    });

    let stream = Box::pin(futures::stream::once(async move {
        Ok(Frame::data(Bytes::from(be_req_msg.to_string())))
    }));

    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/json")
        .body(crate::server::form_stream_body(stream))
        .map_err(|_| error_500())
}

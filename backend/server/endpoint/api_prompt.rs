use http_body_util::BodyExt;
use hyper::{body::Body, header::CONTENT_TYPE, Response, StatusCode};
use serde_json::json;

use crate::{
    cfg::Cfg,
    db::chats::{add_to_chat, get_chat_by_id},
    gpt::gpt_api,
    server::{
        error::{bad_request, internal_error},
        form_body, IncReqst, RGptResp,
    },
};

pub async fn api_prompt(cfg: &Cfg, req: IncReqst) -> RGptResp {
    api_prompt_inner(cfg, req).await.unwrap_or_else(|x| x)
}

pub async fn api_prompt_inner(cfg: &Cfg, req: IncReqst) -> Result<RGptResp, RGptResp> {
    if req.body().size_hint().upper().unwrap_or(u64::MAX) > cfg.max_req_size {
        return Err(bad_request("request body is too large"));
    }

    let bytes = req
        .collect()
        .await
        .map_err(|_| internal_error())?
        .to_bytes()
        .to_vec();
    let prompt = std::str::from_utf8(&bytes).map_err(|_| internal_error())?;

    let (recvd, new_head_id, chat_id) = gpt_api(cfg, prompt).await.map_err(|_| internal_error())?;
    let parsed = serde_json::from_str::<serde_json::Value>(&recvd).map_err(|_| internal_error())?;

    let resp = match parsed
        .get("choices")
        .ok_or_else(internal_error)?
        .get(0)
        .ok_or_else(internal_error)?
        .get("message")
        .ok_or_else(internal_error)?
        .get("content")
        .ok_or_else(internal_error)?
    {
        serde_json::Value::String(string) => String::from(string),
        _ => Err(internal_error())?,
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

    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/json")
        .body(form_body(be_req_msg.to_string()))
        .map_err(|_| internal_error())
}

use crate::cfg::Cfg;
use crate::server::IncReqst;

pub async fn api_prompt(_cfg: &Cfg, req: &IncReqst) -> bool {
    req.uri().path().starts_with("/api/prompt")
}

pub async fn api_chats(_cfg: &Cfg, req: &IncReqst) -> bool {
    req.uri().path().starts_with("/api/chats")
}

pub async fn api_chat_messages(_cfg: &Cfg, req: &IncReqst) -> bool {
    req.uri().path() == "/api/chat/messages"
}

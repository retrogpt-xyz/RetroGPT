use crate::cfg::Cfg;
use crate::server::IncReqst;

pub async fn api_prompt(_cfg: &Cfg, req: &IncReqst) -> bool {
    req.uri().path().starts_with("/api/prompt")
}

pub async fn api_def_sess(_cfg: &Cfg, req: &IncReqst) -> bool {
    req.uri().path().starts_with("/api/get_def_sess")
}

pub async fn serve_static(cfg: &Cfg, req: &IncReqst) -> bool {
    let mut path = cfg.static_dir.join(&req.uri().path()[1..]);
    if path.is_dir() {
        path = path.join("index.html");
    }
    path.is_file()
}

pub async fn auth(_cfg: &Cfg, req: &IncReqst) -> bool {
    req.uri().path().starts_with("/api/auth")
}

pub async fn session(_cfg: &Cfg, req: &IncReqst) -> bool {
    req.uri().path().starts_with("/api/session")
}

pub async fn api_chats(_cfg: &Cfg, req: &IncReqst) -> bool {
    req.uri().path().starts_with("/api/chats")
}

pub async fn api_chat_messages(_cfg: &Cfg, req: &IncReqst) -> bool {
    req.uri().path() == "/api/chat/messages"
}

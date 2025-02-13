use crate::cfg::Cfg;
use crate::server::IncReqst;

pub async fn api_prompt(_cfg: &Cfg, req: &IncReqst) -> bool {
    req.uri().path().starts_with("/api/prompt")
}

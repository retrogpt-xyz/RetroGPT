use crate::cfg::Cfg;
use crate::server::IncReqst;

pub fn api_prompt(_cfg: &Cfg, req: &IncReqst) -> bool {
    req.uri().path().starts_with("/api/prompt")
}

pub fn serve_static(cfg: &Cfg, req: &IncReqst) -> bool {
    let mut path = cfg.static_dir.join(&req.uri().path()[1..]);
    if path.is_dir() {
        path = path.join("index.html");
    }
    path.is_file()
}

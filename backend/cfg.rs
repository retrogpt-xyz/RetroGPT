use std::{env, path::PathBuf};

pub struct Cfg {
    pub static_dir: PathBuf,
    pub api_key: String,
    pub max_req_size: u64,
    pub client: reqwest::Client,
}

impl Cfg {
    pub fn new() -> Self {
        let api_key = env::var("OPENAI_API_KEY").unwrap();
        let static_dir = PathBuf::from("static/");
        let max_req_size = 1024 * 1024;
        let client = reqwest::Client::new();
        Cfg {
            api_key,
            static_dir,
            max_req_size,
            client,
        }
    }
}

use std::{
    env,
    error::Error,
    sync::{Arc, Mutex},
};

use async_openai::config::OpenAIConfig;
use rgpt_db::Database;
use rgpt_stream::StreamRegistry;

/// Shared state between request handler threads
pub struct SharedState {
    pub db: Arc<Database>,

    pub openai_client: async_openai::Client<OpenAIConfig>,

    pub reqwest_client: reqwest::Client,

    pub stream_registry: Mutex<StreamRegistry>,
}

impl SharedState {
    pub async fn new() -> Result<SharedState, Box<dyn Error>> {
        let db = Database::establish_arc().await;

        let api_key = env::var("OPENAI_API_KEY")?;
        let openai_client = async_openai::Client::with_config(
            async_openai::config::OpenAIConfig::new().with_api_key(api_key),
        );
        let reqwest_client = reqwest::Client::new();

        let stream_registry = StreamRegistry::new().into();

        Ok(SharedState {
            db,
            openai_client,
            reqwest_client,
            stream_registry,
        })
    }
}

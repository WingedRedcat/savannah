use std::sync::Arc;

use reqwest::Client;

use crate::{EngineConfig, Navigation, Result};

/// Shared Savannah engine services.
#[derive(Clone)]
pub struct Engine {
    client: Client,
    config: Arc<EngineConfig>,
}

impl Engine {
    /// Creates a validated engine and its reusable HTTP client.
    pub fn new(config: EngineConfig) -> Result<Self> {
        config.validate()?;
        let client = Client::builder()
            .user_agent(config.user_agent.clone())
            .redirect(reqwest::redirect::Policy::limited(10))
            .build()?;
        Ok(Self {
            client,
            config: Arc::new(config),
        })
    }

    /// Starts a cancellable asynchronous navigation.
    pub fn navigation(&self, url: &str) -> Result<Navigation> {
        Navigation::new(self.client.clone(), self.config.clone(), url)
    }

    /// Convenience method that starts and awaits a navigation.
    pub async fn navigate(&self, url: &str) -> Result<crate::LoadedPage> {
        self.navigation(url)?.run().await
    }

    /// Releases engine resources. Cloned handles may keep the HTTP client alive.
    pub async fn shutdown(self) {}
}

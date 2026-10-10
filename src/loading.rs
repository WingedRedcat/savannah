use std::{sync::Arc, time::Duration};

use reqwest::Client;
use tokio_util::sync::CancellationToken;
use url::Url;

use crate::{EngineConfig, Error, Result};

/// Result of a completed navigation.
#[derive(Debug, Clone)]
pub struct LoadedPage {
    pub url: Url,
    pub status: u16,
    pub content_type: Option<String>,
    pub body: Vec<u8>,
}

/// One asynchronous navigation operation.
pub struct Navigation {
    client: Client,
    config: Arc<EngineConfig>,
    url: Url,
    cancellation: CancellationToken,
}

impl Navigation {
    pub(crate) fn new(client: Client, config: Arc<EngineConfig>, raw_url: &str) -> Result<Self> {
        let url = Url::parse(raw_url).map_err(|error| Error::InvalidUrl(error.to_string()))?;
        match url.scheme() {
            "http" | "https" => {}
            scheme => return Err(Error::UnsupportedScheme(scheme.to_owned())),
        }
        Ok(Self {
            client,
            config,
            url,
            cancellation: CancellationToken::new(),
        })
    }

    /// Cancels this navigation. Cancellation is idempotent.
    pub fn cancel(&self) {
        self.cancellation.cancel();
    }

    /// Performs the request without blocking the async runtime.
    pub async fn run(self) -> Result<LoadedPage> {
        let timeout = self.config.navigation_timeout;
        let request = self.client.get(self.url.clone()).send();
        let response = tokio::select! {
            _ = self.cancellation.cancelled() => return Err(Error::Cancelled),
            result = tokio::time::timeout(timeout, request) => result.map_err(|_| Error::Timeout(timeout))??,
        };
        let status = response.status().as_u16();
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        let bytes = tokio::select! {
            _ = self.cancellation.cancelled() => return Err(Error::Cancelled),
            result = tokio::time::timeout(timeout, response.bytes()) => result.map_err(|_| Error::Timeout(timeout))??,
        };
        if bytes.len() > self.config.max_response_bytes {
            return Err(Error::ResponseTooLarge(self.config.max_response_bytes));
        }
        Ok(LoadedPage {
            url: self.url,
            status,
            content_type,
            body: bytes.to_vec(),
        })
    }

    /// Returns the parsed destination URL.
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Returns the configured timeout for this operation.
    pub fn timeout(&self) -> Duration {
        self.config.navigation_timeout
    }
}

use std::time::Duration;

/// Configuration for an [`Engine`](crate::Engine).
#[derive(Clone, Debug)]
pub struct EngineConfig {
    /// Maximum time allowed for one navigation.
    pub navigation_timeout: Duration,
    /// Maximum response body size accepted by the initial loader.
    pub max_response_bytes: usize,
    /// User agent sent by Savannah requests.
    pub user_agent: String,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            navigation_timeout: Duration::from_secs(30),
            max_response_bytes: 16 * 1024 * 1024,
            user_agent: "Savannah/0.1 (+https://github.com/WingedRedcat/savannah)".into(),
        }
    }
}

impl EngineConfig {
    pub(crate) fn validate(&self) -> crate::Result<()> {
        if self.navigation_timeout.is_zero() {
            return Err(crate::Error::InvalidConfiguration(
                "navigation_timeout must be greater than zero",
            ));
        }
        if self.max_response_bytes == 0 {
            return Err(crate::Error::InvalidConfiguration(
                "max_response_bytes must be greater than zero",
            ));
        }
        if self.user_agent.trim().is_empty() {
            return Err(crate::Error::InvalidConfiguration(
                "user_agent must not be empty",
            ));
        }
        Ok(())
    }
}

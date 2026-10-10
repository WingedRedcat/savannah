use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct ExtensionManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub permissions: Vec<ExtensionPermission>,
    #[serde(default)]
    pub content_scripts: Vec<ContentScriptManifest>,
    pub background: Option<ManifestBackground>,
}

impl ExtensionManifest {
    pub fn parse(json: &str) -> Result<Self, ManifestError> {
        let manifest: Self = serde_json::from_str(json)?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn validate(&self) -> Result<(), ManifestError> {
        if self.id.trim().is_empty() {
            return Err(ManifestError::InvalidValue("id is required".to_string()));
        }
        if self.name.trim().is_empty() {
            return Err(ManifestError::InvalidValue("name is required".to_string()));
        }
        if self.version.trim().is_empty() {
            return Err(ManifestError::InvalidValue(
                "version is required".to_string(),
            ));
        }

        let mut seen = HashSet::new();
        for permission in &self.permissions {
            permission.validate()?;
            if !seen.insert(permission.clone()) {
                return Err(ManifestError::InvalidValue(
                    "duplicate extension permission".to_string(),
                ));
            }
        }

        for script in &self.content_scripts {
            script.validate()?;
        }

        if let Some(background) = &self.background {
            background.validate()?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct ManifestBackground {
    pub script: String,
}

impl ManifestBackground {
    fn validate(&self) -> Result<(), ManifestError> {
        if self.script.trim().is_empty() {
            return Err(ManifestError::InvalidValue(
                "background.script is required".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct ContentScriptManifest {
    #[serde(default)]
    pub matches: Vec<String>,
    pub script: String,
}

impl ContentScriptManifest {
    fn validate(&self) -> Result<(), ManifestError> {
        if self.script.trim().is_empty() {
            return Err(ManifestError::InvalidValue(
                "content script path is required".to_string(),
            ));
        }

        if self.matches.is_empty() {
            return Err(ManifestError::InvalidValue(
                "content script must include at least one match origin".to_string(),
            ));
        }

        for origin in &self.matches {
            validate_origin(origin)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, Hash, PartialEq)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum ExtensionPermission {
    Tabs,
    Bookmarks,
    Downloads,
    Messaging,
    Storage,
    Origin(String),
}

impl ExtensionPermission {
    pub fn allows_origin(&self, url: &Url) -> bool {
        match self {
            Self::Origin(origin) => Url::parse(origin)
                .ok()
                .and_then(|parsed| {
                    parsed
                        .host_str()
                        .map(|host| parsed.scheme().to_string() + "://" + host)
                })
                .is_some_and(|allowed_origin| {
                    allowed_origin
                        == format!("{}://{}", url.scheme(), url.host_str().unwrap_or_default())
                }),
            _ => false,
        }
    }

    fn validate(&self) -> Result<(), ManifestError> {
        if let Self::Origin(origin) = self {
            validate_origin(origin)?;
        }
        Ok(())
    }
}

fn validate_origin(origin: &str) -> Result<(), ManifestError> {
    if origin == "<all_urls>" {
        return Err(ManifestError::InvalidValue(
            "<all_urls> is not allowed; use explicit origins".to_string(),
        ));
    }

    let parsed =
        Url::parse(origin).map_err(|error| ManifestError::InvalidValue(error.to_string()))?;
    match parsed.scheme() {
        "http" | "https" => {}
        scheme => {
            return Err(ManifestError::InvalidValue(format!(
                "unsupported origin scheme: {scheme}"
            )))
        }
    }

    if parsed.path() != "/" || parsed.query().is_some() || parsed.fragment().is_some() {
        return Err(ManifestError::InvalidValue(
            "origin permissions must be origin-only URLs".to_string(),
        ));
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum ManifestError {
    #[error("invalid extension manifest value: {0}")]
    InvalidValue(String),
    #[error("invalid extension manifest JSON: {0}")]
    Parse(#[from] serde_json::Error),
}

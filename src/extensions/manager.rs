use std::collections::HashMap;

use url::Url;

use super::{
    manifest::{ExtensionPermission, ManifestError},
    ExtensionManifest, ExtensionMessage, ExtensionStorage, InMemoryExtensionStorage, MessageBus,
};

#[derive(Debug)]
pub struct ExtensionManager<S: ExtensionStorage = InMemoryExtensionStorage> {
    manifests: HashMap<String, ExtensionManifest>,
    message_bus: MessageBus,
    storage: S,
}

impl ExtensionManager<InMemoryExtensionStorage> {
    pub fn new() -> Self {
        Self {
            manifests: HashMap::new(),
            message_bus: MessageBus::new(),
            storage: InMemoryExtensionStorage::new(),
        }
    }
}

impl<S: ExtensionStorage> ExtensionManager<S> {
    pub fn with_storage(storage: S) -> Self {
        Self {
            manifests: HashMap::new(),
            message_bus: MessageBus::new(),
            storage,
        }
    }

    pub fn register_manifest_json(&mut self, json: &str) -> Result<(), ExtensionError> {
        let manifest = ExtensionManifest::parse(json)?;
        self.register_manifest(manifest)
    }

    pub fn register_manifest(&mut self, manifest: ExtensionManifest) -> Result<(), ExtensionError> {
        if self.manifests.contains_key(&manifest.id) {
            return Err(ExtensionError::DuplicateExtension(manifest.id));
        }
        self.manifests.insert(manifest.id.clone(), manifest);
        Ok(())
    }

    pub fn has_permission(
        &self,
        extension_id: &str,
        permission: &ExtensionPermission,
    ) -> Result<bool, ExtensionError> {
        let manifest = self
            .manifests
            .get(extension_id)
            .ok_or_else(|| ExtensionError::UnknownExtension(extension_id.to_string()))?;
        Ok(manifest.permissions.iter().any(|item| item == permission))
    }

    pub fn can_access_origin(&self, extension_id: &str, url: &Url) -> Result<bool, ExtensionError> {
        let manifest = self
            .manifests
            .get(extension_id)
            .ok_or_else(|| ExtensionError::UnknownExtension(extension_id.to_string()))?;

        Ok(manifest.permissions.iter().any(|permission| {
            matches!(permission, ExtensionPermission::Origin(_)) && permission.allows_origin(url)
        }))
    }

    pub fn background_scripts(&self, extension_id: &str) -> Result<Option<&str>, ExtensionError> {
        let manifest = self
            .manifests
            .get(extension_id)
            .ok_or_else(|| ExtensionError::UnknownExtension(extension_id.to_string()))?;
        Ok(manifest
            .background
            .as_ref()
            .map(|entry| entry.script.as_str()))
    }

    pub fn content_scripts_for_origin(
        &self,
        extension_id: &str,
        url: &Url,
    ) -> Result<Vec<&str>, ExtensionError> {
        let manifest = self
            .manifests
            .get(extension_id)
            .ok_or_else(|| ExtensionError::UnknownExtension(extension_id.to_string()))?;
        let origin = format!("{}://{}/", url.scheme(), url.host_str().unwrap_or_default());
        Ok(manifest
            .content_scripts
            .iter()
            .filter(|entry| entry.matches.iter().any(|rule| rule == &origin))
            .map(|entry| entry.script.as_str())
            .collect())
    }

    pub fn send_message(
        &mut self,
        from_extension: &str,
        to_extension: &str,
        channel: &str,
        payload: &str,
    ) -> Result<(), ExtensionError> {
        if !self.has_permission(from_extension, &ExtensionPermission::Messaging)? {
            return Err(ExtensionError::MissingPermission {
                extension_id: from_extension.to_string(),
                permission: ExtensionPermission::Messaging,
            });
        }

        if !self.manifests.contains_key(to_extension) {
            return Err(ExtensionError::UnknownExtension(to_extension.to_string()));
        }

        self.message_bus.send(ExtensionMessage {
            from_extension: from_extension.to_string(),
            to_extension: to_extension.to_string(),
            channel: channel.to_string(),
            payload: payload.to_string(),
        });
        Ok(())
    }

    pub fn take_messages(&mut self, extension_id: &str) -> Vec<ExtensionMessage> {
        self.message_bus.drain_for(extension_id)
    }

    pub fn storage_get(
        &self,
        extension_id: &str,
        key: &str,
        private_mode: bool,
    ) -> Result<Option<String>, ExtensionError> {
        if !self.has_permission(extension_id, &ExtensionPermission::Storage)? {
            return Err(ExtensionError::MissingPermission {
                extension_id: extension_id.to_string(),
                permission: ExtensionPermission::Storage,
            });
        }

        Ok(self.storage.get(extension_id, key, private_mode))
    }

    pub fn storage_set(
        &mut self,
        extension_id: &str,
        key: &str,
        value: String,
        private_mode: bool,
    ) -> Result<(), ExtensionError> {
        if !self.has_permission(extension_id, &ExtensionPermission::Storage)? {
            return Err(ExtensionError::MissingPermission {
                extension_id: extension_id.to_string(),
                permission: ExtensionPermission::Storage,
            });
        }

        self.storage.set(extension_id, key, value, private_mode);
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ExtensionError {
    #[error(transparent)]
    Manifest(#[from] ManifestError),
    #[error("extension already registered: {0}")]
    DuplicateExtension(String),
    #[error("unknown extension: {0}")]
    UnknownExtension(String),
    #[error("extension '{extension_id}' is missing permission '{permission:?}'")]
    MissingPermission {
        extension_id: String,
        permission: ExtensionPermission,
    },
}

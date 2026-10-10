use std::collections::HashMap;

pub trait ExtensionStorage {
    fn get(&self, extension_id: &str, key: &str, private_mode: bool) -> Option<String>;
    fn set(&mut self, extension_id: &str, key: &str, value: String, private_mode: bool);
    fn remove(&mut self, extension_id: &str, key: &str, private_mode: bool);
}

#[derive(Default, Debug)]
pub struct InMemoryExtensionStorage {
    // (extension, private_mode, key)
    values: HashMap<(String, bool, String), String>,
}

impl InMemoryExtensionStorage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ExtensionStorage for InMemoryExtensionStorage {
    fn get(&self, extension_id: &str, key: &str, private_mode: bool) -> Option<String> {
        self.values
            .get(&(extension_id.to_string(), private_mode, key.to_string()))
            .cloned()
    }

    fn set(&mut self, extension_id: &str, key: &str, value: String, private_mode: bool) {
        self.values.insert(
            (extension_id.to_string(), private_mode, key.to_string()),
            value,
        );
    }

    fn remove(&mut self, extension_id: &str, key: &str, private_mode: bool) {
        self.values
            .remove(&(extension_id.to_string(), private_mode, key.to_string()));
    }
}

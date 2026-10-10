use std::collections::VecDeque;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtensionMessage {
    pub from_extension: String,
    pub to_extension: String,
    pub channel: String,
    pub payload: String,
}

#[derive(Default, Debug)]
pub struct MessageBus {
    queue: VecDeque<ExtensionMessage>,
}

impl MessageBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn send(&mut self, message: ExtensionMessage) {
        self.queue.push_back(message);
    }

    pub fn drain_for(&mut self, extension_id: &str) -> Vec<ExtensionMessage> {
        let mut delivered = Vec::new();
        let mut retained = VecDeque::new();

        while let Some(message) = self.queue.pop_front() {
            if message.to_extension == extension_id {
                delivered.push(message);
            } else {
                retained.push_back(message);
            }
        }

        self.queue = retained;
        delivered
    }
}

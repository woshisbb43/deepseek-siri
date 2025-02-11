use async_openai::types::ChatCompletionRequestMessage;
use std::collections::HashMap;

pub struct KVStore {
    store: HashMap<String, Vec<ChatCompletionRequestMessage>>,
}

impl KVStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn add(&mut self, chat_id: &str, message: ChatCompletionRequestMessage) {
        self.store
            .entry(chat_id.to_string())
            .or_insert_with(Vec::new)
            .push(message);
    }

    pub fn get(&self, chat_id: &str) -> Vec<ChatCompletionRequestMessage> {
        self.store
            .get(chat_id)
            .cloned()
            .unwrap_or_default()
    }
} 
use ollama_rs::generation::chat::ChatMessage;

pub struct Memory {
    messages: Vec<ChatMessage>,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
}

impl Memory {
    fn insert_memory() {

    }

    fn get_memory(&self) -> Vec<ChatMessage> {
        self.messages.clone()
    }
}

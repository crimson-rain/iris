/* FILENAME: llm_character.rs
 *
 * DESCRIPTION
 *
 * Responsible for LLM calls and natural language processing.
 * Generating dialogues and quest using system prompt.
 *
 * NOTES
 *
 * AUTHOR:    Rezwan Rahman (RAH22529097)
 * CREATED:   17/11/2024
 * MODIFIED:  17/11/2024
*/

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Memory {
    pub description: String,
    pub timestamp: u64,
    pub access_count: u32,
}

// Memory Structure, used to store short term memory
impl Memory {
    pub fn new(description: String) -> Self {
        Self {
            description,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            access_count: 0,
        }
    }

    pub fn access(&mut self) {
        self.access_count += 1;
    }
}

pub struct MemoryStore {
    memories: HashMap<u64, Memory>,
    next_id: u64,
}

// Memory Storage using a Hasmap
impl MemoryStore {
    pub fn new() -> Self {
        Self {
            memories: HashMap::new(),
            next_id: 0,
        }
    }

    // Add Memory to the Hashamp
    pub fn add_memory(&mut self, description: String) {
        let memory = Memory::new(description);
        self.memories.insert(self.next_id, memory);
        self.next_id += 1;
    }

    // Retrieve the most recent memory
    pub fn retrieve_recent(&self, count: usize) -> Vec<&Memory> {
        let mut memories: Vec<&Memory> = self.memories.values().collect();
        memories.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        memories.into_iter().take(count).collect()
    }

    // Retrieve the most relevant memory.
    pub fn retrieve_relevant(&self, query: &str, count: usize) -> Vec<&Memory> {
        let mut relevant: Vec<&Memory> = self
            .memories
            .values()
            .filter(|memory| memory.description.contains(query))
            .collect();
        relevant.sort_by(|a, b| b.access_count.cmp(&a.access_count));
        relevant.into_iter().take(count).collect()
    }
}

#[cfg(test)]
mod tests {}

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
 * MODIFIED:  18/11/2024
*/

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq)]
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

    pub fn memory_to_str(&self) -> String {
        format!(
            "Description: {}, Timestamp: {}, Access Count: {}",
            self.description, self.timestamp, self.access_count
        )
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
mod tests {
    use super::*;

    fn create_test_memory_store() -> MemoryStore {
        let mut memory_store = MemoryStore::new();
        memory_store.add_memory(format!("Memory 1"));
        memory_store.add_memory(format!("Memory 2"));
        memory_store.add_memory(format!("Memory 3"));
        memory_store
    }

    #[test]
    fn test_memory_struct() {
        let memory = Memory::new(format!("Test Memory Description"));

        assert_eq!(memory.description, format!("Test Memory Description"));
        assert_eq!(memory.access_count, 0);
    }

    #[test]
    fn test_access_count() {
        let mut memory = Memory::new(format!("Test Memory Access Count"));
        memory.access();

        assert_eq!(memory.access_count, 1);
    }

    #[test]
    fn test_memory_store() {
        let memory_store = create_test_memory_store();
        assert_eq!(memory_store.memories.len(), 3);
        assert_eq!(memory_store.next_id, 3)
    }
}

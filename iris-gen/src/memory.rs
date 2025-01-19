//! This module provides functionality for creating and managing NPC memories.
//!  
//! ### Key Components
//! - `Memory`: Represents a single memory with a description timestamp and access count.
//! - `MemoryStore`: Manages multiple memories, allowing for storage, retrieval, and sorting.
//!
//! It defines the `LLM` struct and methods which are associated with it.
//! It is responsible for making API calls to the Ollama API.
//!
//! ### Features
//! - Create and store NPC memories with timestamps.
//! - Retrieve the most recent or relevant memories.
//! - Manage memory access counts to track relevance.
//!
//! This module is intended for systems that simulate NPC behaviors, enabling dynamic memory
//! management and interaction history tracking.

use core::fmt;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a memory with description, timestamp, and access count.
#[derive(Debug, Clone, PartialEq)]
pub struct Memory {
    /// Short description of the memory's content.
    pub description: String,
    /// Timestamp of when the memory was created (seconds since UNIX epoch).
    pub timestamp: u64,
    /// The number of times this memory was accessed.
    pub access_count: u32,
}

// Memory structure represents the memory which is created by parsing description.
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

    /// Increments the Access Count by 1
    pub fn access(&mut self) {
        self.access_count += 1;
    }
}

/// Display method used to print memory and convert to string
impl fmt::Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Description: {}, Timestamp: {}, Access Count: {}",
            self.description, self.timestamp, self.access_count
        )
    }
}

#[derive(Default)]
pub struct MemoryStore {
    memories: HashMap<u64, Memory>,
    next_id: u64,
}

/// Used to store memories
impl MemoryStore {
    /// Add Memory to Hashamp
    pub fn add_memory(&mut self, description: String) {
        // Create memory struct
        let memory = Memory::new(description);
        // Add memory into the hashmap
        self.memories.insert(self.next_id, memory);
        // Increment next id
        self.next_id += 1;
    }

    // Retrieve the most recent memory using timestamp
    pub fn retrieve_recent(&self, count: usize) -> Vec<&Memory> {
        // Retrieve all memories inside the hashmap and place them into a vector
        let mut memories: Vec<&Memory> = self.memories.values().collect();
        // Sort the memories by timestamp
        memories.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        // Collect memories and return vector with reference memory
        memories.into_iter().take(count).collect()
    }

    /// Retrieve the most relevant memory using count
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
        let mut memory_store = MemoryStore::default();
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

    // TODO: Implement more complex memory tests such as using the reterieval functions.
}

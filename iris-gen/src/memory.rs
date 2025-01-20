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

/// A Structure to store memories.
///
/// The purpose of this structure is used to create
/// NPC memories used for generation.
///
/// ### Fields
/// - `description` - The contents of the memory.
/// - `timestamp` - When the memory was created.
/// - `access_count` - Represents the amount of times memory was accessed.
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

    /// Increments the memory by 1 when the memory is accessed.
    pub fn access(&mut self) {
        self.access_count += 1;
    }
}

/// Display method used to convert the memory into a String
impl fmt::Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Description: {}, Timestamp: {}, Access Count: {}",
            self.description, self.timestamp, self.access_count
        )
    }
}

/// A Structure to manage and store memories using a Hashmap.
///
/// This struct provides functionality to store, retrieve, and manage
/// memory entites, each identified by a ID.
///
/// ### Fields
/// - `memories` - A HashMap that stores memory entites, indexed by unique `u64` identifer.
/// - `next_id` - A counter to generate unique IDs for new memory entries.
#[derive(Default)]
pub struct MemoryStore {
    /// A collection of stored memories mapped to their unique IDs.
    memories: HashMap<u64, Memory>,
    /// A counter to generate unique IDs for new memory entries.
    /// It increments each time a new memory is added.
    next_id: u64,
}

impl MemoryStore {
    /// Adds memory into the Hashmap.
    /// ### Arguments
    /// * `text` - A string slice (`&str`) which holds the text to embed.
    ///
    /// ### Returns
    ///
    /// ### Example
    /// ```
    /// let memory_store = MemoryStore::new();
    ///
    /// memory_store.add_memory(foramt!("Memory 1"));
    /// memory_store.add_memory(format!("Memory 2"));
    ///```
    pub fn add_memory(&mut self, description: String) {
        // Create memory struct
        let memory = Memory::new(description);
        // Add memory into the hashmap
        self.memories.insert(self.next_id, memory);
        // Increment next id
        self.next_id += 1;
    }

    /// Retrieve the most recent memory using timestamps
    /// ### Arguments
    /// * `count` - The number of recent memories to return.
    ///
    /// ### Returns
    /// - `Vec<&Memory>` - Returns the a vector of memories.
    /// 
    /// ### Example
    /// ```
    /// let memory_store = MemoryStore::new();
    ///
    /// memory_store.add_memory(foramt!("Memory 1"));
    /// memory_store.add_memory(format!("Memory 2"));
    ///```
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
    fn access_count() {
        let mut memory = Memory::new(format!("Test Memory Access Count"));
        memory.access();

        assert_eq!(memory.access_count, 1);
    }

    #[test]
    fn memory_store() {
        let memory_store = create_test_memory_store();
        assert_eq!(memory_store.memories.len(), 3);
        assert_eq!(memory_store.next_id, 3)
    }

    #[test]
    fn bound_check_retrieve_recent() {
        let memory_store = create_test_memory_store();
        memory_store.retrieve_recent(usize::MAX);
        memory_store.retrieve_recent(usize::MIN);
    }

    #[test]
    fn bound_check_retrieve_relevant() {
        let memory_store = create_test_memory_store();
        memory_store.retrieve_relevant("Memory 1", usize::MAX);
        memory_store.retrieve_relevant("Memory 1", usize::MIN);
    }

    #[test]
    fn retrieve_non_existant_values() {
        let memory_store = create_test_memory_store();
        memory_store.retrieve_relevant("THIS MEMORY DOESN'T EXIST", usize::MAX);
        memory_store.retrieve_relevant("THIS MEMORY DOESN'T EXIST EITHER", usize::MIN);
    }
}

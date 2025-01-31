//! This module provides functionality for creating and managing NPC memories.
//!
//! ### Key Components
//! - `Memory`: Represents a single memory with a description timestamp and access count.
//! - `MemoryStore`: Manages multiple memories, allowing for storage, retrieval, and sorting.
//!
//! ### Features
//! - Create and store NPC memories with timestamps.
//! - Retrieve the most recent or relevant memories.
//! - Manage memory access counts to track relevance.
//!
//! This module is intended for systems that simulate NPC behaviors, enabling dynamic memory
//! management and interaction history tracking.
//!
//! TODO: Implement a Size Limit, Maybe Implement a Method to Forget Memories Etc.

use core::fmt;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
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
                .unwrap_or_else(|_| std::time::Duration::from_secs(0))
                .as_secs(),
            access_count: 0,
        }
    }

    /// Increments the memory by 1 when the memory is accessed.
    pub fn access(&mut self) {
        self.access_count += 1;
    }
}

/// Display Trait used to Convert the Memory Struct to String
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
    /// An AtomicU64 Value is used to safely increment values when multithreading is used.
    next_id: AtomicU64,
}

impl MemoryStore {
    /// Adds memory into the Hashmap.
    ///
    /// ### Arguments
    /// * `text` - A string slice (`&str`) which holds the text to embed.
    ///
    /// ### Returns
    ///
    /// ### Example
    /// ```
    /// use iris_gen::memory::MemoryStore;
    ///
    /// let mut memory_store = MemoryStore::default();
    ///
    /// memory_store.add_memory("Memory 1".to_string());
    /// memory_store.add_memory("Memory 2".to_string());
    ///```
    pub fn add_memory(&mut self, description: String) {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        // Create memory struct
        let memory = Memory::new(description);
        // Add memory into the hashmap
        self.memories.insert(id, memory);
    }

    /// Retrieve the most recent memory using timestamps.
    ///
    /// ### Arguments
    /// * `count` - The number of recent memories to return.
    ///
    /// ### Returns
    /// - `Vec<&Memory>` - Returns the a vector of memories.
    ///
    /// ### Example
    /// ```
    /// use iris_gen::memory::MemoryStore;
    /// use iris_gen::memory::Memory;
    ///
    /// let mut memory_store = MemoryStore::default();
    ///
    /// memory_store.add_memory("Memory 1".to_string());
    /// memory_store.add_memory("Memory 2".to_string());
    ///
    /// let retrieved_memories: Vec<Memory> = memory_store.retrieve_recent(2);
    ///```
    pub fn retrieve_recent(&self, count: usize) -> Vec<Memory> {
        // Retrieve all memories inside the hashmap and place them into a vector
        let mut memories: Vec<Memory> = self.memories.values().cloned().collect();
        // Sort the memories by timestamp
        memories.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        // Collect memories and return vector with reference memory
        memories.into_iter().take(count).collect()
    }

    /// Retrieve the most relevant memory using the access count.
    /// Access count is affected by how many times the memory has been interacted with.
    ///
    /// ### Arguments
    /// * `count` - The number of recent memories to return.
    ///
    /// ### Returns
    /// - `Vec<&Memory>` - Returns the a vector of memories.
    ///
    /// ### Example
    /// ```
    /// use iris_gen::memory::MemoryStore;
    /// use iris_gen::memory::Memory;
    ///
    /// let mut memory_store = MemoryStore::default();
    ///
    /// memory_store.add_memory("Memory 1".to_string());
    /// memory_store.add_memory("Memory 2".to_string());
    ///
    /// let retrieved_memories: Vec<&Memory> = memory_store.retrieve_relevant("Memory", 2);
    ///```
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
        memory_store.add_memory("Memory 1".to_string());
        memory_store.add_memory("Memory 2".to_string());
        memory_store.add_memory("Memory 3".to_string());
        memory_store
    }

    #[test]
    fn test_memory_struct() {
        let memory = Memory::new("Test Memory Description".to_string());

        assert_eq!(memory.description, "Test Memory Description".to_string());
        assert_eq!(memory.access_count, 0);
    }

    #[test]
    fn access_count() {
        let mut memory = Memory::new("Test Memory Access Count".to_string());
        memory.access();

        assert_eq!(memory.access_count, 1);
    }

    #[test]
    fn memory_store() {
        let memory_store = create_test_memory_store();
        assert_eq!(memory_store.memories.len(), 3);
        assert_eq!(memory_store.next_id.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_empty_description() {
        let mut memory_store = MemoryStore::default();
        memory_store.add_memory("".to_string());
        assert_eq!(memory_store.memories.len(), 1);
        assert_eq!(memory_store.memories.get(&0).unwrap().description, "");
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

        let retrieved_memories = memory_store.retrieve_relevant("Memory 1", usize::MAX);
        assert!(!retrieved_memories.is_empty());

        let retrieved_memories = memory_store.retrieve_relevant("Memory 1", usize::MIN);
        assert!(retrieved_memories.is_empty());
    }

    #[test]
    fn retrieve_non_existant_values() {
        let memory_store = create_test_memory_store();
        memory_store.retrieve_relevant("THIS MEMORY DOESN'T EXIST", usize::MAX);
        memory_store.retrieve_relevant("THIS MEMORY DOESN'T EXIST EITHER", usize::MIN);
    }
}

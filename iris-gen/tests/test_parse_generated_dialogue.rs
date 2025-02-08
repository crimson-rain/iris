use iris_gen::agent::orchestrator::Orchestrator;
use iris_gen::memory::MemoryStore;
use ollama_rs::generation::chat::{ChatMessage, MessageRole};

#[tokio::test]
async fn parse_generated_dialogue() {
    // Instantiate LLM
    let mut llm = Orchestrator::default();

    // Create histroy
    let mut hist = Vec::new();
    hist.push(ChatMessage::new(
        MessageRole::User,
        "We are talking about games".to_string(),
    ));

    // Create memory
    let mut memory = MemoryStore::default();
    memory.add_memory("You are a mighty warrior named Chicken".to_string());
    memory.add_memory("You live in Aetheria".to_string());
    memory.add_memory("You are a Knight in Townsville".to_string());

    // Generate response
    let res = llm
        .orchestrate_dialogue(
            &mut hist,
            &mut memory.retrieve_recent(3),
            "Hello, I'm looking to do an adventure",
        )
        .await
        .unwrap();

    assert!(!res.dialogue.is_empty(), "Generated Dialogue is Empty");

    assert!(!res.npc.is_empty(), "Generated NPC Name is Empty");

    assert!(!res.choices.is_empty(), "Generated Choices is Empty");
}

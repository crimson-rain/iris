use iris_gen::llm::LLM;
use iris_gen::memory::MemoryStore;
use iris_gen::model::dialogue::Dialogue;
use iris_gen::utils::parse_json::parse_json;
use ollama_rs::generation::chat::{ChatMessage, MessageRole};

#[tokio::test]
async fn parse_generated_dialogue() {
    // Instantiate LLM
    let mut llm = LLM::default();

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
        .generate_dialogue(
            "Hello, I'm looking to do an adventure",
            &mut hist,
            &mut memory.retrieve_recent(3),
        )
        .await;

    // Parse value to json
    let parsed_json = parse_json(res.unwrap().message.content.as_str()).unwrap();

    // Serialize json to dialogue struct
    let dia_struct = Dialogue::try_from(parsed_json.as_str()).unwrap();

    assert!(
        !dia_struct.dialogue.is_empty(),
        "Generated Dialogue is Empty"
    );

    assert!(!dia_struct.npc.is_empty(), "Generated NPC Name is Empty");

    assert!(!dia_struct.choices.is_empty(), "Generated Choices is Empty");
}

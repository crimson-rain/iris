use iris_gen::llm::LLM;
use iris_gen::memory::MemoryStore;
use iris_gen::model::dialogue::Dialogue;
use ollama_rs::generation::chat::{ChatMessage, MessageRole};

#[tokio::test]
async fn parse_generated_dialogue() {

    let mut llm = LLM::default();

    let mut hist = Vec::new();
    hist.push(ChatMessage::new(
        MessageRole::User,
        "We are talking about games".to_string(),
    ));

    let mut memory = MemoryStore::default();
    memory.add_memory("You are a mighty warrior named Chicken".to_string());
    memory.add_memory("You live in Aetheria".to_string());
    memory.add_memory("You are a Knight in Townsville".to_string());

    let res = llm
        .generate_dialogue(
            "Hello, I'm looking to do an adventure",
            &mut hist,
            &mut memory.retrieve_recent(3),
        )
    .await;
    
    let dia_struct = Dialogue::try_from(res.unwrap().message.content.as_str()).unwrap();

    assert!(!dia_struct.dialogue.is_empty());
    assert!(!dia_struct.npc.is_empty());
    assert!(!dia_struct.choices.is_empty());
}
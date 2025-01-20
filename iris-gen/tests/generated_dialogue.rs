use iris_gen::llm::LLM;
use iris_gen::memory::MemoryStore;
use ollama_rs::generation::chat::{ChatMessage, MessageRole};

#[tokio::test]
async fn test_generate_dialogue() {
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

    println!("{:?}", res.as_ref().unwrap().message.content);

    assert!(res.is_ok(), "Dialogue Generation Failed: {:?}", res);

    assert_eq!(hist.len(), 3, "Chat History Wasn't Updated");
    println!("Last Message: {:?}", hist.last().unwrap().content);
    assert_eq!(
        hist.last().unwrap().content,
        res.unwrap().message.content,
        "Chat History Message Was Not Updated Successfully"
    );
}

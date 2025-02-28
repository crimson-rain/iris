use iris_gen::agent::orchestrator::Orchestrator;
use iris_gen::memory::MemoryStore;

#[tokio::test]
async fn test_orchestrate_dialogue() {
    let mut orchestrator = Orchestrator::default();

    let mut hist = Vec::new();

    let mut memory = MemoryStore::default();
    memory.add_memory("My name is Chicken".to_string());
    memory.add_memory("I Live in Aetheria".to_string());
    memory.add_memory("I live in Townsville".to_string());

    let res = orchestrator
        .orchestrate_dialogue(
            &mut hist,
            &mut memory.retrieve_recent(3),
            "What colour is the sky?",
        )
        .await
        .unwrap();

    println!("{:?}", res);

    assert!(
        !hist.is_empty(),
        "Chat History should not be empty after adding a message."
    );

    assert_eq!(hist.len(), 3, "Chat History Wasn't Updated");
}

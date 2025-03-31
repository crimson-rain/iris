//! `agent/tools.rs`
//!
//! WIP - Work in Progress: To be Completed.
//! - Implement a larger Variety.

#![allow(unused)]

/// Get the temperature and weather for a given city.
///
/// * city - City to get the weather for.
#[ollama_rs_macros::function]
pub async fn get_weather(city: String) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
    Ok(reqwest::get(format!("https://wttr.in/{city}?format=%C+%t"))
        .await?
        .text()
        .await?)
}

/// Get Foo and returns bar.
#[ollama_rs_macros::function]
pub async fn get_foo() -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
    Ok("LLM Generated Bar!".to_string())
}

#[cfg(test)]
mod tests {
    use std::vec;

    use ollama_rs::Ollama;
    use ollama_rs::coordinator::Coordinator;
    use ollama_rs::generation::chat::ChatMessage;

    use crate::agent::configs::LLM_MODEL;

    use super::*;

    #[tokio::test]
    async fn test_tool() {
        let ollama = Ollama::default();
        let history = vec![];
        let tools = ollama_rs_macros::tool_group![get_weather, get_foo];
        let mut coordinator =
            Coordinator::new_with_tools(ollama, LLM_MODEL.to_string(), history, tools);

        let user_messages = vec!["Can I get the weather for a city please?", "Dhaka"];

        for user_message in user_messages {
            println!("User: {user_message}");

            let user_message = ChatMessage::user(user_message.to_owned());
            let resp = coordinator.chat(vec![user_message]).await;
            println!("Assistant: {}", resp.unwrap().message.content);
        }
    }
}

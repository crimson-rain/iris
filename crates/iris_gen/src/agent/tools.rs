//! `agent/tools.rs`

#![allow(unused)]

/// Makes an API call to a Weather API to get the weather for a given city.
///
/// * city - City to get the weather for.
#[ollama_rs_macros::function]
pub async fn get_weather(city: String) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
    Ok(reqwest::get(format!("https://wttr.in/{city}?format=%C+%t"))
        .await?
        .text()
        .await?)
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
        let tools = ollama_rs_macros::tool_group![get_weather];
        let mut coordinator =
            Coordinator::new(ollama, LLM_MODEL.to_string(), history).add_tool(get_weather);

        let user_messages = vec!["Can I get the weather for a city please?", "Roehampton"];

        for user_message in user_messages {
            println!("User: {user_message}");

            let user_message = ChatMessage::user(user_message.to_owned());
            let resp = coordinator.chat(vec![user_message]).await;
            println!("Assistant: {}", resp.unwrap().message.content);
        }
    }
}

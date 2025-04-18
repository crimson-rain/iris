//! `agent/maestro.rs`
//! Maestro is responsible for making LLM calls as well as handling various other
//! tasks, is required to create the needed operation.

use super::configs::DIALOGUE_SYSTEM_PROMPT;
use super::model::Model;
use crate::error::IrisGenError;
use crate::rag::RAG;
use ollama_rs::generation::chat::ChatMessage;

#[derive(Clone)]
pub struct Maestro {
    model: Model,
    history: Vec<ChatMessage>,
}

impl Default for Maestro {
    fn default() -> Self {
        let system_message = ChatMessage::new(
            ollama_rs::generation::chat::MessageRole::System,
            DIALOGUE_SYSTEM_PROMPT.to_string(),
        );

        Self {
            model: Model::default(),
            history: vec![system_message],
        }
    }
}

impl Maestro {
    pub async fn conduct_dialogue_gen(&mut self, prompt: String) -> Result<String, IrisGenError> {
        let rag_res = self.conduct_rag(&prompt).await?;

        let rag_inject_prompt = format!("CONTEXT: {}, PROMPT: {}", rag_res, prompt);

        let resp = self
            .model
            .generate_request(&rag_inject_prompt, &mut self.history)
            .await?;

        dbg!(&self.history);

        Ok(resp.message.content)
    }

    //    pub async fn conduct_dialogue_gen_with_tools(&mut self, prompt: String) -> Result<String, IrisGenError> {
    //        let rag_res = self.conduct_rag(&prompt).await?;
    //
    //        let rag_inject_prompt = format!("CONTEXT: {}, PROMPT: {}", rag_res, prompt);
    //
    //        let resp = self
    //            .model
    //            .generate_request_with_tools(&rag_inject_prompt, self.history.clone())
    //            .await?;
    //
    //        self.history.push(ChatMessage::new(
    //            ollama_rs::generation::chat::MessageRole::User,
    //            prompt.clone(),
    //        ));
    //
    //        self.history.push(ChatMessage::new(
    //            ollama_rs::generation::chat::MessageRole::Assistant,
    //            resp.message.content.clone(),
    //        ));
    //
    //        dbg!(&self.history);
    //
    //        Ok(resp.message.content)
    //    }

    pub async fn conduct_quest_gen(&self) -> Result<String, IrisGenError> {
        Ok("Conducted Quest Generation".to_string())
    }

    pub async fn conduct_embed_gen(&self, data: String) -> Result<Vec<Vec<f32>>, IrisGenError> {
        let embeds = self.model.generate_embeddings(&data).await?;
        Ok(embeds.embeddings)
    }

    pub async fn conduct_rag(&self, prompt: &String) -> Result<String, IrisGenError> {
        let rag = RAG::new().await;
        let _ = rag.init_collection(self).await?;
        let _ = rag.init_world_collection(self).await?;

        let npc_rag_resp = rag.rag_resp(self, prompt.to_string()).await?;
        let world_rag_resp = rag.world_rag_resp(self, prompt.to_string()).await?;

        let resp = format!("NPC_RAG: {}, WORLD_RAG: {}", npc_rag_resp, world_rag_resp);

        Ok(resp)
    }
}

// NOTE: History is working fine here.
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_conduct_dialogue_gen() {
        let mut maestro = Maestro::default();

        assert!(
            maestro
                .conduct_dialogue_gen("How are you?, Mel".to_string())
                .await
                .is_ok()
        );

        dbg!(maestro.history);
    }

    #[tokio::test]
    async fn test_conduct_quest_gen() {
        let maestro = Maestro::default();
        assert!(maestro.conduct_quest_gen().await.is_ok())
    }

    #[tokio::test]
    async fn test_conduct_dialogue_gen_with_history() {
        let mut maestro = Maestro::default();

        maestro
            .conduct_dialogue_gen("What is your name?".to_string())
            .await
            .unwrap();

        maestro
            .conduct_dialogue_gen("And whats the weather like in Roehampton?".to_string())
            .await
            .unwrap();

        maestro
            .conduct_dialogue_gen("What was the place I asked about?".to_string())
            .await
            .unwrap();

        assert!(maestro.history.len() > 3);
        assert!(maestro.history[0].role == ollama_rs::generation::chat::MessageRole::System);
        assert!(maestro.history[1].role == ollama_rs::generation::chat::MessageRole::User);
        assert!(maestro.history[2].role == ollama_rs::generation::chat::MessageRole::Assistant);
    }
}

//! `utils/mod.rs`
//!
//! General purpose utility/helper functions used in the project.

use ollama_rs::Ollama;
use ollama_rs::models::{LocalModel, ModelInfo};

use crate::error::IrisGenError;

pub async fn list_ollama_models(ollama: &Ollama) -> Result<Vec<LocalModel>, IrisGenError> {
    let res = ollama.list_local_models().await?;
    Ok(res)
}

pub async fn load_model_info(
    ollama: &Ollama,
    model_name: String,
) -> Result<ModelInfo, IrisGenError> {
    let res = ollama.show_model_info(model_name).await?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::agent::configs::LLM_MODEL;

    use super::*;

    #[tokio::test]
    async fn list_models() {
        let ollama = Ollama::default();

        let models = list_ollama_models(&ollama).await.unwrap();
        println!("{:?}", models)
    }

    #[tokio::test]
    async fn get_model_info() {
        let ollama = Ollama::default();

        let models = load_model_info(&ollama, LLM_MODEL.to_string())
            .await
            .unwrap();
        println!("{:?}", models.template)
    }
}

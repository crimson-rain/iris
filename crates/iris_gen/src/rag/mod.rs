//! `rag/mod.rs`
//!
//! All the RAG logic needed to create similiary search and setting up to make requests.
//! To prompt more accurate and narrow dialogue generation.

#![allow(dead_code)]

use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    CreateCollectionBuilder, PointStruct, UpsertPointsBuilder,
    VectorParamsBuilder,
};

use tokio::fs;

use crate::error::IrisGenError;

async fn connect_to_qdrant() -> Result<(), IrisGenError> {
    let _client = Qdrant::from_url("http://localhost:6334")
        .skip_compatibility_check()
        .build()?;

    Ok(())
}

async fn create_collection() -> Result<(), IrisGenError> {
    let client = Qdrant::from_url("http://localhost:6334")
        .skip_compatibility_check()
        .build()?;

    if client.collection_exists("test_collection").await? {
        let _ = client.delete_collection("test_collection").await?;
    }

    client
        .create_collection(
            CreateCollectionBuilder::new("test_collection").vectors_config(
                VectorParamsBuilder::new(4, qdrant_client::qdrant::Distance::Dot),
            ),
        )
        .await?;

    println!("Collection Created Successfully");

    Ok(())
}

async fn add_vectors(data: Vec<String>, embeds: Vec<f32>) -> Result<(), IrisGenError> {
    let client = Qdrant::from_url("http://localhost:6334")
        .skip_compatibility_check()
        .build()?;

    let points = vec![PointStruct::new(
        42,
        vec![0.0_f32; 768],
        [
            ("great", true.into()),
            ("level", 9000.into()),
            ("text", "Hi Qdrant".into()),
            ("list", vec![1.234f32, 0.815].into()),
        ],
    )];

    let response = client
        .upsert_points(UpsertPointsBuilder::new("test_collection", points))
        .await?;

    dbg!(response);
    Ok(())
}

async fn collection_info() -> Result<(), IrisGenError> {
    let client = Qdrant::from_url("http://localhost:6334")
        .skip_compatibility_check()
        .build()?;

    let collection_list = client.list_collections().await?;

    dbg!(collection_list);

    Ok(())
}

async fn create_npc_collection() -> Result<(), IrisGenError> {
    let client = Qdrant::from_url("http://localhost:6334")
        .skip_compatibility_check()
        .build()?;

    if client.collection_exists("npc_collection").await? {
        return Ok(());
    }

    Ok(())
}

async fn load_npc_data(folder: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut npc_data = Vec::new();
    let mut dir = fs::read_dir(folder).await?;

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        if path.is_file() {
            let content = fs::read_to_string(&path).await?;
            npc_data.push(content);
        }
    }

    Ok(npc_data)
}

#[cfg(test)]
mod tests {
    use qdrant_client::qdrant::SearchPointsBuilder;

    use crate::agent::maestro;

    use super::*;

    #[tokio::test]
    async fn test_qdrant() {
        assert!(connect_to_qdrant().await.is_ok());
    }

    #[tokio::test]
    async fn test_collection() {
        assert!(create_collection().await.is_ok());
    }

    #[tokio::test]
    async fn list_collections() {
        assert!(collection_info().await.is_ok());
    }

    #[tokio::test]
    async fn test_load_data() {
        let folder_dir = "../../../iris_data/npc_data";
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

        let path = format!("{}{}", manifest_dir, folder_dir);

        let npc_data = load_npc_data(&path).await;

        assert!(npc_data.is_ok())
    }

    #[tokio::test]
    async fn test_load_data_to_qdrant() {
        // Load NPC Data
        let folder_dir = "../../../iris_data/npc_data";
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

        let path = format!("{}{}", manifest_dir, folder_dir);
        
        let npc_data = load_npc_data(&path).await.unwrap();

        // Create NPC Collection in Qdrant
        let client = Qdrant::from_url("http://localhost:6334")
            .skip_compatibility_check()
            .build().unwrap();
        
        let  _ = client
            .create_collection(CreateCollectionBuilder::new("npc_knowledge_base").vectors_config(
                    VectorParamsBuilder::new(768, qdrant_client::qdrant::Distance::Cosine)
                    ),
                )
            .await;

        // Prepare Qdrant Data
        let maestro = maestro::Maestro::default();

        let mut points = Vec::new();

        for (i, data) in npc_data.iter().enumerate() {
            let embeds = maestro.conduct_embed_gen(data.to_string()).await.unwrap();

            assert_eq!(embeds[0].len(), 768, "Embedding Size Mismatch");

            let point = qdrant_client::qdrant::PointStruct::new(
                i as u64, 
                embeds[0].clone(), 
                [("text", data.clone().into())]
            );

            points.push(point)
        }

        let resp = client
            .upsert_points(qdrant_client::qdrant::UpsertPointsBuilder::new("npc_knowledge_base", points))
            .await
            .unwrap();

        dbg!(resp);

        let query_text = "Mel";

        let query_vector = maestro.conduct_embed_gen(query_text.to_string()).await.unwrap();

        let search_request = SearchPointsBuilder::new(
            "npc_knowledge_base", 
            query_vector[0].clone(), 
            1,
        ).with_payload(true);

        let response = client.search_points(search_request).await.unwrap();

        dbg!(response);
    }
}

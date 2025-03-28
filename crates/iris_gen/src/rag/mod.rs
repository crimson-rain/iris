//! `rag/mod.rs`
//!
//! All the RAG logic needed to create similiary search and setting up to make requests.
//! To prompt more accurate and narrow dialogue generation.

#![allow(dead_code)]

use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    CreateCollection, CreateCollectionBuilder, Distance, PointStruct, UpsertPointsBuilder, VectorParamsBuilder
};
use tokio::fs;
use tokio::io::AsyncReadExt;

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

    add_vectors().await?;

    Ok(())
}

async fn add_vectors() -> Result<(), IrisGenError> {
    let client = Qdrant::from_url("http://localhost:6334")
        .skip_compatibility_check()
        .build()?;

    let points = vec![PointStruct::new(
        42,
        vec![0.0_f32; 512],
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
        return Ok(())
    }



    Ok(())
} 


async fn load_npc_data(folder: &str) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let mut npc_data = Vec::new();
    let mut dir = fs::read_dir(folder).await?;

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        if path.is_file() {
            let mut file = fs::File::open(&path).await?;
            let mut content = String::new();
            file.read_to_string(&mut content).await?;
            let filename = path.file_name().unwrap().to_string_lossy().to_string();
            npc_data.push((filename, content));
        }
    }

    Ok(npc_data)
}

#[cfg(test)]
mod tests {
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
}

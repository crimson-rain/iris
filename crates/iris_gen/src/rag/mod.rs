//! `rag/mod.rs`
//!
//! All the RAG logic needed to create similiary search and setting up to make requests.
//! To prompt more accurate and narrow dialogue generation.

#![allow(dead_code)]

use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    CreateCollectionBuilder, PointStruct, UpsertPointsBuilder, VectorParamsBuilder,
};

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
}

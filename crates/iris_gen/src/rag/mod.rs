use qdrant_client::qdrant::{CreateCollectionBuilder, VectorParamsBuilder};
use qdrant_client::{Qdrant, QdrantError};

async fn connect_to_qdrant() -> Result<(), QdrantError> {
    let _client = Qdrant::from_url("http://localhost:6334").build()?;

    Ok(())
}

async fn create_collection() -> Result<(), Box<dyn std::error::Error>> {
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

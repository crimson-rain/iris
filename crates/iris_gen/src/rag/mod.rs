use qdrant_client::{Qdrant, QdrantError};

async fn connect_to_qdrant() -> Result<(), QdrantError> {
    let client = Qdrant::from_url("http://localhost:6334")
        .build();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_qdrant() {
        assert!(connect_to_qdrant().await.is_ok());
    }
}

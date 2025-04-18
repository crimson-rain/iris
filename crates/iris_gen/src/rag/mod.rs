//! `rag/mod.rs`
//!
//! All the RAG logic needed to create similiary search and setting up to make requests.
//! To prompt more accurate and narrow dialogue generation.

#![allow(unused)]

mod data_scehmas;

use data_scehmas::{NPCData, WorldData};
use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    CreateCollectionBuilder, PointStruct, SearchPointsBuilder, UpsertPointsBuilder,
    VectorParamsBuilder,
};

use tokio::fs;

use crate::agent::maestro::Maestro;
use crate::error::IrisGenError;

pub struct RAG {
    client: qdrant_client::Qdrant,
}

impl RAG {
    pub async fn new() -> Self {
        Self {
            client: connect_to_qdrant().await.unwrap(),
        }
    }

    pub async fn init_collection(&self, maestro: &Maestro) -> Result<(), IrisGenError> {
        if self.client.collection_exists("npc_collection").await? {
            return Ok(());
        }

        if self.client.collection_exists("world_collection").await? {
            return Ok(());
        }

        // NPC ---------------------------------------------------------------------

        // Load NPC data
        let folder_dir = "../../../iris_data/npc_data";
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

        let path = format!("{}{}", manifest_dir, folder_dir);

        let npc_data = load_data(&path).await.unwrap();

        // Create Collection
        let _ = self
            .client
            .create_collection(
                CreateCollectionBuilder::new("npc_collection").vectors_config(
                    VectorParamsBuilder::new(768, qdrant_client::qdrant::Distance::Cosine),
                ),
            )
            .await?;

        let _ = self
            .client
            .create_collection(
                CreateCollectionBuilder::new("world_collection").vectors_config(
                    VectorParamsBuilder::new(768, qdrant_client::qdrant::Distance::Cosine),
                ),
            )
            .await?;

        // Prepare Points from NPC Data
        let mut points = Vec::new();

        for (i, data) in npc_data.iter().enumerate() {
            let embeds = maestro.conduct_embed_gen(data.to_string()).await?;

            let npc: NPCData = serde_json::from_str(&data)?;

            let npc_information = serde_json::to_string(&npc.npc_information)?;
            let notable_traits = npc.notable_traits.clone();
            let relationships = serde_json::to_string(&npc.relationships.clone())?;
            let equipment = serde_json::to_string(&npc.equipment.clone())?;

            let point = qdrant_client::qdrant::PointStruct::new(
                npc.id,
                embeds[0].clone(),
                [
                    ("title", npc.title.into()),
                    ("npc_information", npc_information.into()),
                    ("notable_traits", notable_traits.into()),
                    ("background", npc.background.into()),
                    ("relationships", relationships.into()),
                    ("equipment", equipment.into()),
                ],
            );

            points.push(point);
        }


        let resp = self
            .client
            .upsert_points(UpsertPointsBuilder::new("npc_collection", points))
            .await?;

        // Location ---------------------------------------------------------------------
        
        let mut points = Vec::new();

        // Load World Data
        let folder_dir = "../../../iris_data/world_data";
        let path = format!("{}{}", manifest_dir, folder_dir);

        let world_data = load_data(&path).await?;

        println!("{:?}", world_data.len());

        for (i ,data) in world_data.iter().enumerate() {
            let embeds = maestro.conduct_embed_gen(data.to_string()).await?;

            let location: WorldData = serde_json::from_str(&data)?;
            let tag = serde_json::to_string(&location.tag)?;

            let coordinates = serde_json::to_string(&location.coordinates)?;

            let point = qdrant_client::qdrant::PointStruct::new(
                location.id, 
                embeds[0].clone(), 
                [
                    ("title", location.title.into()),
                    ("tag", tag.into()),
                    ("coordinates", coordinates.into()),
                    ("description", location.description.into())
                ],
            );
            
            points.push(point);
        }

        let resp = self
            .client
            .upsert_points(UpsertPointsBuilder::new("world_collection", points))
            .await?;

        println!(" INSERTING WORLD DATA: {:?}", resp);

        Ok(())
    }

    pub async fn rag_resp(
        &self,
        maestro: &Maestro,
        prompt: String,
    ) -> Result<String, IrisGenError> {
        let query_embeds = maestro.conduct_embed_gen(prompt.to_string()).await?;

        let npc_search_request = SearchPointsBuilder::new(
            "npc_collection", 
            query_embeds[0].clone(), 
            2
        ).with_payload(true);

        let world_search_request = SearchPointsBuilder::new(
            "world_collection", 
            query_embeds[0].clone(), 
            1
        ).with_payload(true);

        let npc_response = self.client.search_points(npc_search_request).await?;
        let world_response = self.client.search_points(world_search_request).await?;

        Ok(format!("NPC_RAG_RESULT: {:?}, WORLD_RAG_RESULT: {:?}", npc_response, world_response))
    }
}

async fn connect_to_qdrant() -> Result<qdrant_client::Qdrant, IrisGenError> {
    let client = Qdrant::from_url("http://localhost:6334")
        .skip_compatibility_check()
        .build()?;

    Ok(client)
}

async fn load_data(folder: &str) -> Result<Vec<String>, IrisGenError> {
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
    use crate::agent::maestro::{self, Maestro};

    use super::RAG;

    #[tokio::test]
    async fn test_init_collection() {
        let maestro = Maestro::default();
        let rag = RAG::new().await;

        rag.init_collection(&maestro);

        let resp = rag.rag_resp(&maestro, "Where is the Adventurer Guild?".to_string()).await.unwrap();

        dbg!(resp);
    }
}

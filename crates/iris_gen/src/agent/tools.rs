//! `agent/tools.rs`

use serde::{Serialize, Deserializer};

/// Reterieve the weather for a specified city.
///
/// *city - The city for which to get the weather.
#[ollama_rs_macros::function]
async fn get_weather(city: String) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://wttr.in/{city}?format=%C+%t");
    let response = reqwest::get(&url).await?.text().await?;
    Ok(response)
}

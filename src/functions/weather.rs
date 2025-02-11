use async_openai::types::FunctionDefinition;
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "https://api.openweathermap.org/data/2.5/weather";

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherParams {
    pub longitude: f64,
    pub latitude: f64,
}

pub fn get_weather_function() -> FunctionDefinition {
    FunctionDefinition {
        name: "get_weather".to_string(),
        description: Some("Get the current weather".to_string()),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "longitude": {
                    "type": "number",
                    "description": "The longitude to get the weather for"
                },
                "latitude": {
                    "type": "number",
                    "description": "The latitude to get the weather for"
                }
            },
            "required": ["longitude", "latitude"]
        }),
    }
}

pub async fn get_weather(
    lon: f64,
    lat: f64,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("{}?lat={}&lon={}&appid={}", BASE_URL, lat, lon, api_key);
    let response = reqwest::get(&url).await?;
    
    if !response.status().is_success() {
        return Err(format!("HTTP error! status: {}", response.status()).into());
    }
    
    let data = response.text().await?;
    Ok(data)
} 
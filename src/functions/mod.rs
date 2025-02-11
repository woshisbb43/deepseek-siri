use async_openai::types::FunctionDefinition;
use serde_json::Value;
use crate::AppState;

mod weather;

pub struct FunctionHandler;

impl FunctionHandler {
    pub fn get_functions() -> Vec<FunctionDefinition> {
        vec![weather::get_weather_function()]
    }

    pub async fn handle(
        name: &str,
        args: &str,
        state: &AppState,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match name {
            "get_weather" => {
                let params: weather::WeatherParams = serde_json::from_str(args)?;
                weather::get_weather(
                    params.longitude,
                    params.latitude,
                    &state.openweathermap_api_key,
                ).await
            }
            _ => Ok("Function not found".to_string()),
        }
    }
} 
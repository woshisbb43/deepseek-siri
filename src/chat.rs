use async_openai::{
    types::{ChatCompletionRequestMessage, CreateChatCompletionRequest, Role},
    Client,
};
use serde::{Deserialize, Serialize};
use crate::functions::FunctionHandler;

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub chat_id: String,
    pub input: String,
    pub date: String,
    pub location: Location,
}

pub async fn handle_chat(state: crate::AppState, req: ChatRequest) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new()
        .with_api_key(state.groq_api_key.clone())
        .with_api_base("https://api.groq.com/openai/v1/");

    let system_message = format!(
        "You are Siri Pro. Answer in 1-2 sentences. Be friendly, helpful and concise. \
         Default to metric units when possible. Keep the conversation short and sweet. \
         You only answer in text. Don't include links or any other extras. \
         Don't respond with computer code, for example don't return user longitude.\n\n\
         User's current info:\n\
         date: {}\n\
         lat:{}, lon:{}",
        req.date, req.location.latitude, req.location.longitude
    );

    let mut chat_history = state.kv_store.lock().await;
    chat_history.add(&req.chat_id, ChatCompletionRequestMessage {
        role: Role::User,
        content: Some(req.input.clone()),
        ..Default::default()
    });

    let mut messages = vec![
        ChatCompletionRequestMessage {
            role: Role::System,
            content: Some(system_message),
            ..Default::default()
        },
    ];
    messages.extend(chat_history.get(&req.chat_id));

    let request = CreateChatCompletionRequest {
        model: "llama3-70b-8192".to_string(),
        messages: messages.clone(),
        tools: Some(FunctionHandler::get_functions()),
        ..Default::default()
    };

    let response = client.chat().create(request).await?;
    let choice = response.choices.first().ok_or("No response generated")?;

    if let Some(tool_calls) = &choice.message.tool_calls {
        // Handle function calls
        for tool_call in tool_calls {
            let result = FunctionHandler::handle(
                &tool_call.function.name,
                &tool_call.function.arguments,
                &state,
            ).await?;
            
            chat_history.add(&req.chat_id, ChatCompletionRequestMessage {
                role: Role::Function,
                name: Some(tool_call.function.name.clone()),
                content: Some(result),
                ..Default::default()
            });
        }
    }

    let content = choice.message.content.clone().unwrap_or_default();
    chat_history.add(&req.chat_id, ChatCompletionRequestMessage {
        role: Role::Assistant,
        content: Some(content.clone()),
        ..Default::default()
    });

    Ok(content)
} 
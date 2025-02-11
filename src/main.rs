use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration; // 新增导入

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct Message {
    content: String,
    reasoning_content: Option<String>,
}

#[tokio::main]
async fn main() {
    // 从环境变量获取 API Key
    let api_key = "sk-1001361df8ad43e884bc766fb2474a8f";

    // 使用带超时设置的 Client
    let client = Client::builder()
        .timeout(Duration::from_secs(40)) // 设置超时时间为 30 秒
        .build()
        .expect("Failed to build client");

    let url = "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions";

    // 构建消息内容
    let messages = vec![ChatMessage {
        role: "user".to_string(),
        content: "猫和狗谁好".to_string(),
    }];

    // 发起请求
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "deepseek-v3",
            "messages": messages,
        }))
        .send()
        .await
        .expect("Failed to send request");

    // 调试：打印原始响应文本
    let response_text = response.text().await.expect("Failed to read response text");
    println!("Raw response: {}", response_text);

    // 使用解析后的响应继续后续操作
    let chat_response: ChatResponse = serde_json::from_str(&response_text)
        .expect("Failed to parse response");

    // 打印思考过程和最终答案
    if let Some(reasoning) = &chat_response.choices[0].message.reasoning_content {
        println!("思考过程：");
        println!("{}", reasoning);
    }

    println!("最终答案：");
    println!("{}", chat_response.choices[0].message.content);
}

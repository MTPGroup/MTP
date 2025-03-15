use reqwest::header::AUTHORIZATION;

use crate::model::{ChatCompletion, ChatCompletionRequest, Message, MessageData};

/// 向 DeepSeek API 发送聊天请求并获取回复
///
/// # 参数
/// * `messages` - 聊天历史消息列表，包含用户和系统的对话内容
///
/// # 返回值
/// * `Result<Message, Box<dyn std::error::Error>>` - 成功时返回 AI 的回复消息，失败时返回错误
///
/// # 错误
/// 此函数可能在以下情况返回错误：
/// * 环境变量 `DEEPSEEK_API_KEY` 未设置
/// * API 请求发送失败
/// * 响应解析失败
///
/// # 示例
/// ```
/// let messages = vec![
///     MessageData {
///         role: "system".to_string(),
///         content: "你是一个有用的助手".to_string(),
///     },
///     MessageData {
///         role: "user".to_string(),
///         content: "你好".to_string(),
///     },
/// ];
/// let response = chat(messages).await?;
/// println!("{}", response.content);
/// ```
pub async fn chat_with_ds(
    messages: Vec<MessageData>,
    api_key: Option<String>,
) -> Result<Message, Box<dyn std::error::Error>> {
    let model = "deepseek-reasoner";
    let base_url = "https://api.deepseek.com/chat/completions";
    let api_key = match api_key {
        Some(key) => key,
        None => "".to_string(), // 默认 API Key
    };
    let authorization = format!("Bearer {}", api_key);

    let request_body = ChatCompletionRequest {
        messages,
        model: model.to_string(),
        stream: false,
    };

    let client = reqwest::Client::new();

    // 打印请求信息
    println!("发送请求到: {}", base_url);
    // println!("请求体: {:?}", request_body);

    // 发送请求并获取原始响应
    let response = client
        .post(base_url)
        .header(AUTHORIZATION, authorization)
        .json(&request_body)
        .send()
        .await?;

    // 检查响应状态
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await?;
        return Err(format!("API请求失败: 状态码 {}, 错误信息: {}", status, error_text).into());
    }

    // 获取响应文本用于调试
    let response_text = response.text().await?;
    // println!("API响应: {}", response_text);

    // 尝试解析响应
    let completion: ChatCompletion = match serde_json::from_str(&response_text) {
        Ok(completion) => completion,
        Err(e) => {
            return Err(format!("解析响应失败: {}. 原始响应: {}", e, response_text).into());
        }
    };

    if completion.choices.is_empty() {
        return Err("API返回的choices为空".into());
    }

    Ok(completion.choices[0].message.clone())
}

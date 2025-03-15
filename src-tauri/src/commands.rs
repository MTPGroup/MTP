use crate::db;
use entity::{conversation, message};
use llm::model::MessageData;
use tauri::State;

// 数据库相关命令
#[tauri::command]
pub async fn get_conversations(
    db_client: State<'_, db::DbClient>,
) -> Result<Vec<db::ConversationWithStudent>, String> {
    db::get_conversations(&db_client)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_conversation_by_id(
    id: String,
    db_client: State<'_, db::DbClient>,
) -> Result<Option<db::ConversationWithStudent>, String> {
    db::get_conversation_by_id(&db_client, id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_conversation(
    data: db::ConversationData,
    db_client: State<'_, db::DbClient>,
) -> Result<conversation::Model, String> {
    db::create_conversation(&db_client, data)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_conversation(
    id: String,
    data: db::ConversationUpdateData,
    db_client: State<'_, db::DbClient>,
) -> Result<conversation::Model, String> {
    db::update_conversation(&db_client, id, data)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_conversation(
    id: String,
    db_client: State<'_, db::DbClient>,
) -> Result<conversation::Model, String> {
    db::delete_conversation(&db_client, id)
        .await
        .map_err(|e| e.to_string())
}

// 消息相关命令
#[tauri::command]
pub async fn get_messages_by_conversation_id(
    conversation_id: String,
    db_client: State<'_, db::DbClient>,
) -> Result<Vec<message::Model>, String> {
    db::get_messages_by_conversation_id(&db_client, conversation_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_messages_by_conversation_id_with_pagination(
    conversation_id: String,
    page: u64,
    page_size: u64,
    db_client: State<'_, db::DbClient>,
) -> Result<Vec<message::Model>, String> {
    db::get_messages_by_conversation_id_with_pagination(
        &db_client,
        conversation_id,
        page,
        page_size,
    )
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_message(
    data: db::MessageData,
    db_client: State<'_, db::DbClient>,
) -> Result<message::Model, String> {
    db::create_message(&db_client, data)
        .await
        .map_err(|e| e.to_string())
}

// 聊天相关命令
#[tauri::command]
pub async fn chat_with_llm(
    message: MessageData,
    conversation_id: String,
    db_client: State<'_, db::DbClient>,
) -> Result<MessageData, String> {
    // 获取对话历史
    let mut history = db::get_messages_by_conversation_id(&db_client, conversation_id.clone())
        .await
        .map_err(|e| e.to_string())?;

    // 检查是否存在system消息
    let has_system_message = history.iter().any(|msg| msg.role == "system");

    // 如果没有system消息，则创建一个
    if !has_system_message {
        println!("未找到system消息，创建默认system消息");
        let conversation = db::get_conversation_by_id(&db_client, conversation_id.clone())
            .await
            .map_err(|e| e.to_string())?;
        let student = conversation.unwrap().student;
        let system_message_data = db::MessageData {
            conversation_id: conversation_id.clone(),
            role: "system".to_string(),
            content: student.prompt.clone(),
            name: None,
            index: Some(0), // 确保system消息排在最前面
        };

        // 将system消息保存到数据库
        let system_message = db::create_message(&db_client, system_message_data)
            .await
            .map_err(|e| e.to_string())?;

        // 将系统消息添加到历史记录中
        history.insert(0, system_message);
    }

    // 构造消息列表，包含历史消息和新消息
    let mut messages = Vec::new();

    // 检查是否需要合并连续的用户消息
    let mut merged_content = message.content.clone();
    let mut skip_indices = Vec::new();

    // 从后向前查找连续的用户消息
    if !history.is_empty() && message.role == "user" {
        let mut i = history.len() - 1;
        while i < history.len() && history[i].role == "user" {
            merged_content = format!("{}\n{}", history[i].content, merged_content);
            skip_indices.push(i);
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }

    // 添加历史消息，但跳过需要合并的用户消息
    for (i, msg) in history.iter().enumerate() {
        if skip_indices.contains(&i) {
            continue; // 跳过需要合并的用户消息
        }
        messages.push(MessageData {
            role: msg.role.clone(),
            content: msg.content.clone(),
        });
    }

    // 添加合并后的新消息
    let merged_message = MessageData {
        role: message.role.clone(),
        content: merged_content.clone(),
    };
    messages.push(merged_message);

    // 处理消息列表，确保没有连续的相同角色消息
    let mut processed_messages = Vec::new();
    let mut prev_role = String::new();

    for msg in &messages {
        // 如果当前消息与前一条消息角色相同，则合并内容
        if !processed_messages.is_empty() && msg.role == prev_role {
            let last_index = processed_messages.len() - 1;
            let last_msg: &MessageData = &processed_messages[last_index];
            let merged_content = format!("{}\n\n{}", last_msg.content.clone(), msg.content.clone());

            // 替换最后一条消息
            processed_messages[last_index] = MessageData {
                role: msg.role.clone(),
                content: merged_content,
            };
        } else {
            // 添加新消息
            processed_messages.push(MessageData {
                role: msg.role.clone(),
                content: msg.content.clone(),
            });
            prev_role = msg.role.clone();
        }
    }

    println!(
        "原始消息数量: {}, 处理后的消息数量: {}",
        messages.len(),
        processed_messages.len()
    );

    // 将用户消息保存到数据库
    let user_message_data = db::MessageData {
        conversation_id: conversation_id.clone(),
        role: message.role,
        content: message.content,
        name: None,
        index: None,
    };

    db::create_message(&db_client, user_message_data)
        .await
        .map_err(|e| e.to_string())?;

    // 调用 llm crate 中的聊天功能，使用处理后的消息列表
    let response = match llm::chat::chat_with_ds(processed_messages).await {
        Ok(r) => r,
        Err(e) => {
            let error_msg = format!("LLM聊天失败: {}", e);
            println!("[ERROR] {}", error_msg);
            return Err(error_msg);
        }
    };

    println!("LLM响应: {:?}", response);

    // 创建消息数据
    let message_data = db::MessageData {
        conversation_id,
        role: response.role.clone(),
        content: response.content.clone(),
        name: None,
        index: None,
    };

    // 将回复保存到数据库
    db::create_message(&db_client, message_data)
        .await
        .map_err(|e| e.to_string())?;

    // 返回响应
    Ok(MessageData {
        role: response.role,
        content: response.content,
    })
}

use entity::prelude::{Conversation, Message, Student};
use entity::{conversation, message, student};
use migration::MigratorTrait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

pub type DbClient = Arc<Mutex<DatabaseConnection>>;

// 用于序列化和反序列化的结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationData {
    pub id: Option<String>,
    pub title: Option<String>,
    pub student_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationUpdateData {
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentWithConversation {
    #[serde(flatten)]
    pub student: student::Model,
    pub conversation: Option<conversation::Model>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationWithStudent {
    #[serde(flatten)]
    pub conversation: conversation::Model,
    pub student: student::Model,
}

// 消息相关结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageData {
    pub conversation_id: String,
    pub role: String,
    pub content: String,
    pub name: Option<String>,
    pub index: Option<i32>,
}

// 初始化数据库客户端
pub async fn init_db() -> Result<DbClient, DbErr> {
    // 使用应用数据目录而不是相对路径
    let app_dir =
        std::env::current_dir().map_err(|e| DbErr::Custom(format!("无法获取当前目录: {}", e)))?;

    // 确保数据目录存在
    let db_dir = app_dir.join("data");
    std::fs::create_dir_all(&db_dir)
        .map_err(|e| DbErr::Custom(format!("无法创建数据目录: {}", e)))?;

    let db_path = db_dir.join("db.sqlite");
    println!("数据库路径: {}", db_path.display());
    // 检查数据库文件是否存在，如果不存在则创建空文件
    if !db_path.exists() {
        println!("数据库文件不存在，将创建新数据库");
        std::fs::File::create(&db_path)
            .map_err(|e| DbErr::Custom(format!("无法创建数据库文件: {}", e)))?;
    }

    let db_url = format!("sqlite://{}", db_path.display());
    // let db_url = "sqlite://./db.sqlite";
    let conn = sea_orm::Database::connect(db_url).await?;

    // 运行迁移以确保表结构存在
    migration::Migrator::up(&conn, None).await?;

    // 检查是否需要初始化数据
    let student_count = Student::find().all(&conn).await?.len();

    if student_count == 0 {
        // 数据库中没有学生数据，需要初始化
        initialize_database(&conn).await?;
    }

    Ok(Arc::new(Mutex::new(conn)))
}

// 初始化数据库数据
async fn initialize_database(conn: &DatabaseConnection) -> Result<(), DbErr> {
    println!("开始初始化数据库...");

    // 获取学生数据
    println!("正在获取学生数据...");
    let base_url = "https://arona.hanasaki.tech/api/student";

    let response = reqwest::get(base_url)
        .await
        .map_err(|e| DbErr::Custom(format!("Failed to fetch student data: {}", e)))?;

    let data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| DbErr::Custom(format!("Failed to parse student data: {}", e)))?;

    // 学生默认prompt模板
    let get_default_prompt = |student_name: &str| -> String {
        format!(
            "你是来自蔚蓝档案的学生{}，你应该表现得符合角色特性。\n\
            你需要保持角色的一致性，友好地与用户交流。\n\
            在对话中要展现出{}的性格特点和说话方式。\n\
            请记住，你是在与用户私聊，要有亲切感。",
            student_name, student_name
        )
    };

    // 导入每个学生并创建对应私聊
    if let Some(students) = data.get("data").and_then(|d| d.as_array()) {
        for student_data in students {
            let student_name = student_data["PersonalName"]
                .as_str()
                .unwrap_or_default()
                .to_string();
            let student_id = student_data["Id"].as_u64().unwrap_or_default().to_string();
            println!("正在导入学生 {}...", &student_id);
            let avatar_url = format!(
                "https://aronacdn.hanasaki.tech/images/student/icon/{}.webp",
                student_id
            );

            // 查找学生是否已存在
            let existing_student = Student::find()
                .filter(student::Column::Name.eq(&student_name))
                .one(conn)
                .await?;

            if let Some(existing) = existing_student {
                // 如果学生已存在，更新avatars数组
                let mut avatars = Vec::new();

                // 尝试解析现有的avatars JSON
                if let Ok(parsed_avatars) = serde_json::from_str::<Vec<String>>(&existing.avatars) {
                    avatars = parsed_avatars;
                } else if !existing.avatars.is_empty() {
                    // 如果解析失败，说明可能不是JSON格式，将其作为单个元素
                    avatars.push(existing.avatars.clone());
                }

                // 检查是否已存在相同的avatar
                if !avatars.contains(&avatar_url) {
                    avatars.push(avatar_url);
                }

                // 更新学生记录
                let mut student_model: student::ActiveModel = existing.into();
                student_model.avatars = Set(serde_json::to_string(&avatars).unwrap_or_default());

                student_model.update(conn).await?;

                println!("已更新学生 {} 的头像数组", student_name);
            } else {
                // 创建新学生记录，avatars作为JSON数组
                let avatars = vec![avatar_url];
                let avatars_json = serde_json::to_string(&avatars).unwrap_or_default();

                let student_model = student::ActiveModel {
                    id: Default::default(), // 自动生成ID
                    name: Set(student_name.clone()),
                    avatars: Set(avatars_json),
                    prompt: Set(get_default_prompt(&student_name)),
                };

                let student = student_model.insert(conn).await?;

                // 为每个学生创建对话
                let uuid = uuid::Uuid::new_v4().to_string();
                let now = chrono::Utc::now().naive_utc().and_utc().fixed_offset();

                let conversation = conversation::ActiveModel {
                    id: Set(uuid),
                    created_at: Set(now),
                    updated_at: Set(now),
                    title: Set(student.name.clone()),
                    student_name: Set(student.name.clone()),
                };

                conversation.insert(conn).await?;

                println!("已创建学生 {} 及对应对话", student.name);
            }
        }
    }

    println!("数据库初始化完成！");
    Ok(())
}

// 获取所有对话
pub async fn get_conversations(client: &DbClient) -> Result<Vec<ConversationWithStudent>, DbErr> {
    let conn = client.lock().await;

    let conversations = Conversation::find()
        .order_by_desc(conversation::Column::UpdatedAt)
        .all(&*conn)
        .await?;

    let mut result = Vec::new();

    for conversation_model in conversations {
        let student = conversation_model
            .find_related(Student)
            .one(&*conn)
            .await?
            .expect("Student should exist");

        result.push(ConversationWithStudent {
            conversation: conversation_model,
            student,
        });
    }

    Ok(result)
}

// 根据ID获取对话
pub async fn get_conversation_by_id(
    client: &DbClient,
    id: String,
) -> Result<Option<ConversationWithStudent>, DbErr> {
    let conn = client.lock().await;

    let conversation = Conversation::find_by_id(id).one(&*conn).await?;

    if let Some(conversation_model) = conversation {
        let student = conversation_model
            .find_related(Student)
            .one(&*conn)
            .await?
            .expect("Student should exist");

        Ok(Some(ConversationWithStudent {
            conversation: conversation_model,
            student,
        }))
    } else {
        Ok(None)
    }
}

// 创建新对话
pub async fn create_conversation(
    client: &DbClient,
    data: ConversationData,
) -> Result<conversation::Model, DbErr> {
    let conn = client.lock().await;

    let uuid = uuid::Uuid::new_v4().to_string();

    let now = chrono::Utc::now().naive_utc();

    let conversation = conversation::ActiveModel {
        id: Set(uuid),
        created_at: Set(now.and_utc().fixed_offset()),
        updated_at: Set(now.and_utc().fixed_offset()),
        title: Set(data.title.unwrap_or_default()),
        student_name: Set(data.student_name),
    };

    let result = conversation.insert(&*conn).await?;
    Ok(result)
}

// 更新对话
pub async fn update_conversation(
    client: &DbClient,
    id: String,
    data: ConversationUpdateData,
) -> Result<conversation::Model, DbErr> {
    let conn = client.lock().await;

    let conversation = Conversation::find_by_id(id.clone())
        .one(&*conn)
        .await?
        .ok_or_else(|| DbErr::Custom("Conversation not found".to_string()))?;

    let mut conversation: conversation::ActiveModel = conversation.into();

    if let Some(title) = data.title {
        conversation.title = Set(title);
    }

    conversation.updated_at = Set(chrono::Utc::now().naive_utc().and_utc().fixed_offset());

    let result = conversation.update(&*conn).await?;
    Ok(result)
}

// 删除对话
pub async fn delete_conversation(
    client: &DbClient,
    id: String,
) -> Result<conversation::Model, DbErr> {
    let conn = client.lock().await;

    let conversation = Conversation::find_by_id(id.clone())
        .one(&*conn)
        .await?
        .ok_or_else(|| DbErr::Custom("Conversation not found".to_string()))?;

    let result = conversation.clone();
    conversation.delete(&*conn).await?;

    Ok(result)
}

// 获取对话的所有消息
pub async fn get_messages_by_conversation_id(
    client: &DbClient,
    conversation_id: String,
) -> Result<Vec<message::Model>, DbErr> {
    let conn = client.lock().await;

    let messages = Message::find()
        .filter(message::Column::ConversationId.eq(conversation_id))
        .order_by_asc(message::Column::Index)
        .all(&*conn)
        .await?;

    Ok(messages)
}

// 获取对话的消息（带分页）
pub async fn get_messages_by_conversation_id_with_pagination(
    client: &DbClient,
    conversation_id: String,
    page: u64,
    page_size: u64,
) -> Result<Vec<message::Model>, DbErr> {
    let conn = client.lock().await;

    let messages = Message::find()
        .filter(message::Column::ConversationId.eq(conversation_id))
        .order_by_asc(message::Column::Index)
        .paginate(&*conn, page_size)
        .fetch_page(page)
        .await?;

    Ok(messages)
}

// 创建新消息
pub async fn create_message(client: &DbClient, data: MessageData) -> Result<message::Model, DbErr> {
    let conn = client.lock().await;

    // 获取当前对话中最大的索引值
    let max_index = Message::find()
        .filter(message::Column::ConversationId.eq(data.conversation_id.clone()))
        .order_by_desc(message::Column::Index)
        .one(&*conn)
        .await?
        .map(|m| m.index)
        .unwrap_or(-1);

    let now = chrono::Utc::now().naive_utc().and_utc().fixed_offset();

    let message = message::ActiveModel {
        id: Default::default(), // 自动生成ID
        conversation_id: Set(data.conversation_id),
        role: Set(data.role),
        content: Set(data.content),
        name: Set(data.name.unwrap_or_default()),
        created_at: Set(now),
        index: Set(data.index.unwrap_or(max_index + 1)),
    };

    // 更新对话的更新时间
    let result = message.insert(&*conn).await?;
    Ok(result)
}

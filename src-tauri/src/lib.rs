// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;

mod commands;
mod db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_webview_window("main").unwrap();
                if tauri::is_dev() {
                    window.open_devtools();
                }
            }
            // 初始化数据库
            let handle = app.handle();
            tauri::async_runtime::block_on(async {
                let db_client = db::init_db().await.expect("Failed to initialize database");
                handle.manage(db_client);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_conversations,
            commands::get_conversation_by_id,
            commands::create_conversation,
            commands::update_conversation,
            commands::delete_conversation,
            commands::get_messages_by_conversation_id,
            commands::get_messages_by_conversation_id_with_pagination,
            commands::create_message,
            commands::chat_with_llm // 添加新的聊天命令
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

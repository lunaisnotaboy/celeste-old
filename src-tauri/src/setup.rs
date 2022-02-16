use rand::Rng;
use crate::structures::{Database};

async fn generate_random() -> u8 {
    let mut rng = rand::thread_rng();
    return rng.gen();
}

#[tauri::command]
pub async fn setup(invoke_message: String, window: tauri::Window, database: tauri::State<'_, Database>) -> Result<String, String> {
    println!("Called from {}", window.label());
    println!("Message from JS: {}", invoke_message);

    let rng = generate_random().await;
    if rng >= 127 {
        Err("Random fail".into())
    } else {
        Ok("Random success".into())
    }
}

#[tauri::command]
pub fn cmd_b() -> String {
    "Command B".to_string()
}

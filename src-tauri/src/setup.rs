use rand::Rng;
use crate::structures::{Database};
use crate::config::read_config;

async fn generate_random() -> u8 {
    let mut rng = rand::thread_rng();
    return rng.gen();
}

#[tauri::command]
pub async fn cmd_test(invoke_message: String, window: tauri::Window, database: tauri::State<'_, Database>) -> Result<String, String> {
    println!("Called from {}", window.label());
    println!("Message from JS: {}", invoke_message);

    let rng = generate_random().await;
    if rng >= 127 {
        Err("Random fail".into())
    } else {
        Ok("Random success".into())
    }
}

pub fn setup() {
    println!("Initializing...");
    if read_config().getbool("app.networking", "offline").unwrap().unwrap() {
        println!("Starting up in offline mode");
    }

    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("Done initializing.");
}

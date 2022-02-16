use crate::config::read_config;
use rand::Rng;
use log::{debug, error, warn, info};

async fn generate_random() -> u8 {
    let mut rng = rand::thread_rng();
    return rng.gen();
}

#[tauri::command]
pub async fn cmd_test(invoke_message: String, window: tauri::Window) -> Result<String, String> {
    debug!("Called from {}", window.label());
    debug!("Message from JS: {}", invoke_message);

    let rng = generate_random().await;
    if rng >= 127 {
        Err("Random fail".into())
    } else {
        Ok("Random success".into())
    }
}

pub fn setup() {
    info!("Initializing...");
    if read_config()
        .getbool("app.networking", "offline")
        .unwrap()
        .unwrap()
    {
        info!("Starting up in offline mode");
    }

    std::thread::sleep(std::time::Duration::from_secs(2));
    info!("Done initializing.");
}

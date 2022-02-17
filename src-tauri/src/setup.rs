use crate::config::read_config;

// Main setup function
pub async fn setup() {
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

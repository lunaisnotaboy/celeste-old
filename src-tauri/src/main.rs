#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod setup;
mod structures;

fn main() {
  tauri::Builder::default()
      .manage(structures::Database {})
      .invoke_handler(tauri::generate_handler![
                      setup::setup,
                      setup::cmd_b
                    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


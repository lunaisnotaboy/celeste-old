#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;
mod setup;
mod structures;

fn main() {
  tauri::Builder::default()
      .setup(|app| {
          let splashscreen_window = app.get_window("splashscreen").unwrap();
          let main_window = app.get_window("main").unwrap();
          // Perfom initialization code on new task so the app doesn't freeze
          tauri::async_runtime::spawn(async move {
                setup::setup();

                // After it's done, close the splashscreen and display main window
                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
            });
          Ok(())
        })
      .manage(structures::Database {})
      .invoke_handler(tauri::generate_handler![
                      setup::cmd_test,
                    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


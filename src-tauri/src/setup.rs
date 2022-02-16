#[tauri::command]
fn setup(invoke_message: String) -> String {
    println!("hewwo");
    println!("Message: {}", invoke_message);

    return "Nyaa~".into();
}

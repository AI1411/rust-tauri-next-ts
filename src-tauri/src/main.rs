use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct MyMessage {
    field_str: String,
    field_u32: u32,
}

#[tauri::command]
fn command_with_object(message: MyMessage) -> MyMessage {
    let MyMessage { field_str, field_u32 } = message;

    MyMessage {
        field_str: format!("{}", field_str),
        field_u32: field_u32 + 1,
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command_with_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

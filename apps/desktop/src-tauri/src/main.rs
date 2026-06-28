mod commands;
mod kernel;

use commands::get_foundation_state;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_foundation_state])
        .run(tauri::generate_context!())
        .expect("failed to run DeepSeek Agent OS desktop app");
}

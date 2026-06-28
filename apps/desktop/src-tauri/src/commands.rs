use crate::kernel::models::FoundationState;

#[tauri::command]
pub fn get_foundation_state() -> FoundationState {
    FoundationState::default()
}

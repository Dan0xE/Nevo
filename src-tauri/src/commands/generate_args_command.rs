use crate::utils::generate_args::generate_args;

#[tauri::command]
pub(crate) fn generate_args_command() -> String {
    match generate_args() {
        Ok(_) => "sucess".to_string(),
        Err(e) => e.to_string(),
    }
}

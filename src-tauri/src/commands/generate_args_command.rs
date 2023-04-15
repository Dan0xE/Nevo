use crate::utils::generate_args::generate_args;

#[tauri::command]
pub(crate) fn generate_args_command() -> Result<String, String> {
    match generate_args() {
        Ok(_) => Ok("sucess".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

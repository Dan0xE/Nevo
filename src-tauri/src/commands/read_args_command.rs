use crate::utils::read_arguments::read_args;

#[tauri::command]
pub fn read_args_command() -> Result<Vec<String>, String> {
    match read_args() {
        Ok(args) => Ok(args),
        Err(e) => Err(e.to_string()),
    }
}

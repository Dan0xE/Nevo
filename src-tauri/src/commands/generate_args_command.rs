use crate::utils::generate_args::generate_args;

#[tauri::command]
pub(crate) fn generate_args_command() -> bool {
    let result = generate_args();
    return result;
}

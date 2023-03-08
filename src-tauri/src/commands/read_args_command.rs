use crate::utils::read_arguments::read_args;

#[tauri::command]
pub(crate) fn read_args_command() -> Vec<String> {
    let args = read_args();
    args
}

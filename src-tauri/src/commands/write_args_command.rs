use crate::utils::write_arguments::write_args;

#[tauri::command]
pub(crate) fn write_args_command(args: String) {
    write_args(args);
}

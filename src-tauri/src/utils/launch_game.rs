use super::create_shortcut::create_symlimk;
use std::{env, process::Command, time::Duration};
use tauri::api::path::desktop_dir;

pub(crate) fn launch_game() {
    let curr_path = env::current_dir().unwrap();
    let arg_file_path = curr_path.join("args.bat");

    let arg_buf_str = arg_file_path
        .into_os_string()
        .to_string_lossy()
        .into_owned();
    let binding = desktop_dir();
    let desktop_dir = binding
        .as_ref()
        .map(|p| p.to_str().unwrap_or(""))
        .unwrap_or("");

    create_symlimk(&arg_buf_str, desktop_dir, "nevo_shortcut").unwrap();

    Command::new("cmd")
        .arg("/c")
        .arg(curr_path.join("args.bat"))
        .spawn()
        .unwrap();
}

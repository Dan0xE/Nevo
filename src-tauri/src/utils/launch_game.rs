use super::create_shortcut::create_symlimk;
use std::{env, io, process::Command};
use tauri::api::path::desktop_dir;

pub(crate) fn launch_game() -> io::Result<String> {
    let curr_path = env::current_dir().unwrap_or_else(|e| {
        println!("Error getting current directory: {}", e);
        std::process::exit(1);
    });
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

    create_symlimk(&arg_buf_str, desktop_dir, "nevo_shortcut").unwrap_or_else(|e| {
        println!("Error creating shortcut: {}", e);
        // Err(format!("Failed to create symlink: {:?}", e));
    });

    //does that even make sense?
    match Command::new("cmd")
        .arg("/c")
        .arg(curr_path.join("args.bat"))
        .spawn()
    {
        Ok(_) => Ok("Game Launched"),
        Err(e) => Err(format!("Failed to launch game: {:?}", e)),
    };

    Ok("sucess".to_string())
}

use super::create_shortcut::create_symlimk;
use std::{env, path::Path, process::Command};
use tauri::api::path::desktop_dir;

pub(crate) fn launch_game() -> Result<String, String> {
    let curr_path = env::current_dir().unwrap();

    if Path::new("args.bat").exists() {
        let arg_buf_str = curr_path
            .join("args.bat")
            .into_os_string()
            .to_string_lossy()
            .into_owned();

        let binding = desktop_dir();
        let desktop_dir = binding.as_ref().map(|p| p.to_str().unwrap()).unwrap();

        create_symlimk(&arg_buf_str, &desktop_dir, "nevo_shortcut").unwrap_or_else(|e| {
            println!("Error creating shortcut: {}", e);
            Err::<String, String>(format!("Failed to create Symlink: {:?}", e)).unwrap();
        });

        match Command::new("cmd")
            .arg("/c")
            .arg(curr_path.join("args.bat"))
            .spawn()
        {
            Ok(_) => Ok::<String, String>("Game Launched".to_string()).unwrap(),
            Err(e) => Err::<String, String>(format!("Failed to launch game: {:?}", e)).unwrap(),
        };
    } else {
        return Err("args.bat does not exist".to_string());
    }
    return Ok("sucess".to_string());
}

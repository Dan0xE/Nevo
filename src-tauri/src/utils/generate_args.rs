use powershell_script::run;
use std::env::current_dir;

use super::is_minecraft_running::is_minecraft_running;

pub(crate) fn generate_args() -> bool {
    let current_dir = current_dir().unwrap_or_default();

    let args_txt_path = current_dir.join("args.txt");
    let args_bat_path = current_dir.join("args.bat");

    if args_txt_path.exists() {
        std::fs::remove_file(args_txt_path).unwrap();
    }
    if args_bat_path.exists() {
        std::fs::remove_file(args_bat_path).unwrap();
    }

    let file_path = current_dir.join("p.ps1");
    if is_minecraft_running() {
        match run(file_path.into_os_string().into_string().unwrap().as_str()) {
            Ok(output) => {
                println!("{}", output);
                true
            }
            Err(e) => {
                println!("Error while generating args: {}", e);
                false
            }
        }
    } else {
        println!("Please start minecraft");
        false
    }
}

use std::env::current_dir;

use super::{argument_wrapper::argument_wrapper, is_minecraft_running::is_minecraft_running};

pub(crate) fn generate_args() -> Result<String, String> {
    let current_dir = current_dir().unwrap();

    let args_txt_path = current_dir.join("args.txt");
    let args_bat_path = current_dir.join("args.bat");

    if args_txt_path.exists() {
        std::fs::remove_file(args_txt_path).unwrap();
    }
    if args_bat_path.exists() {
        std::fs::remove_file(args_bat_path).unwrap();
    }

    if is_minecraft_running() && argument_wrapper().is_ok() {
        println!("minecraft is running & wrapper ran successfully");
        Ok::<String, String>("sucess".to_string()).unwrap();
    } else {
        println!("Argument wrapper failed to run or minecraft is not running");
        return Err::<String, String>("Failed to generate arguments, read the log".to_string());
    }
    Ok("sucess".to_string())
}

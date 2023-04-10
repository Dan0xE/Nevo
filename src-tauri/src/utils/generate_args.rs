use std::env::current_dir;

use super::{argument_wrapper::argument_wrapper, is_minecraft_running::is_minecraft_running};

pub(crate) fn generate_args() -> bool {
    let current_dir = current_dir().unwrap_or_default();

    let args_txt_path = current_dir.join("args.txt");
    let args_bat_path = current_dir.join("args.bat");

    if args_txt_path.exists() {
        std::fs::remove_file(args_txt_path).unwrap_or_else(|e| {
            println!("Error removing args.txt file: {}", e);
        });
    }
    if args_bat_path.exists() {
        std::fs::remove_file(args_bat_path).unwrap_or_else(|e| {
            println!("Error removing args.bat file: {}", e);
        });
    }

    if is_minecraft_running() && argument_wrapper() {
        println!("minecraft is running & wrapper ran successfully");
        true
    } else {
        println!("Argument wrapper failed to run or minecraft is not running");
        false
    }
}

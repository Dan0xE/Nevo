use std::env::current_dir;

use super::{argument_wrapper::argument_wrapper, is_minecraft_running::is_minecraft_running};

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

    if is_minecraft_running() {
        if argument_wrapper() {
            println!("Argument wrapper ran successfully");
            true
        } else {
            println!("Argument wrapper failed");
            false
        }
    } else {
        println!("Minecraft is not running");
        false
    }
}

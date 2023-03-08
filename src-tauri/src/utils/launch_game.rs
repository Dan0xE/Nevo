use std::{env, process::Command};

pub(crate) fn launch_game() {
    let curr_path = env::current_dir().unwrap();
    let arg_file_path = curr_path.join("args.bat");

    Command::new("cmd")
        .arg("/c")
        .arg(arg_file_path)
        .spawn()
        .unwrap();
}

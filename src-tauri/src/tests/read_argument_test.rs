use std::{
    fs::{self, File},
    path::Path,
    time::Duration,
};

use crate::commands::read_args_command::read_args_command;

#[test]
fn read_argument_fail_file_does_not_exist() {
    std::thread::sleep(Duration::from_secs(3));
    if Path::new("args.txt").exists() {
        fs::remove_file("args.bat").unwrap();
    }
    let result = read_args_command();
    assert!(result.is_err());
}

#[test]
fn read_arguments_sucess() {
    std::thread::sleep(Duration::from_secs(6));
    File::create("args.txt").unwrap();
    let result = read_args_command();
    assert!(result.is_ok());

    //cleanup
    fs::remove_file("args.txt").unwrap();
}

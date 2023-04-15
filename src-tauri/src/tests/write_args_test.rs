use std::{fs, time::Duration};

use crate::commands::write_args_command::write_args_command;

#[test]
fn write_args_command_test() {
    std::thread::sleep(Duration::from_secs(2));
    let result = write_args_command("test".to_string());
    assert!(result.is_ok());

    //cleanup
    fs::remove_file("args.bat").unwrap();
}

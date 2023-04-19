use std::fs::{self, File};

use crate::commands::copy_snapshot_command::copy_snapshot_command;

#[test]
fn copy_snapshot_fail_not_a_directory() {
    //checking proper error handling
    let result = copy_snapshot_command("non_existant".to_string(), "non_existant".to_string());
    assert!(result.is_err());
}

#[test]
fn copy_snapshot_sucess() {
    fs::create_dir_all("test").unwrap();
    fs::create_dir_all("test_dest").unwrap();
    File::create("test\\foo.txt").unwrap();

    let result = copy_snapshot_command("test".to_string(), "test_dest".to_string());
    assert!(result.is_ok());

    //cleanup
    fs::remove_dir_all("test").unwrap();
    fs::remove_dir_all("test_dest").unwrap();
}

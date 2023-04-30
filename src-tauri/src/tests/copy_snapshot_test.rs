use std::fs::{self, File};

use crate::utils::copy_snapshot::copy_snapshot;

#[test]
fn copy_snapshot_fail_not_a_directory() {
    let result = copy_snapshot("non_existant", "non_existant");
    assert!(result.is_err());
}

#[test]
fn copy_snapshot_sucess() {
    fs::create_dir_all("test").unwrap();
    fs::create_dir_all("test_dest").unwrap();
    File::create("test\\foo.txt").unwrap();

    let result = copy_snapshot("test", "test_dest");
    assert!(result.is_ok());

    //cleanup
    fs::remove_dir_all("test").unwrap();
    fs::remove_dir_all("test_dest").unwrap();
}

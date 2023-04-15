use advisory_lock::{AdvisoryFileLock, FileLockError, FileLockMode};
use std::{
    env,
    fs::{self, File},
    time,
};

use crate::commands::copy_snapshot_command::copy_snapshot_command;

#[test]
fn copy_snapshot_fail_not_a_directory() {
    let result = copy_snapshot_command("non_existant".to_string(), "non_existant".to_string());
    assert!(result.len() > 0); //We check this to know if the function panicked (which it shouldnt)
    assert_eq!(result, "Source or destination dir not found"); //we expect this error message
}

#[test]
fn copy_snapshot_fail_due_to_process_lock() {
    //intitialize strings
    let mut current_dir = env::current_dir().unwrap();
    let mut current_dir_2 = env::current_dir().unwrap();
    let mut current_dir_3 = env::current_dir().unwrap();

    //we create a folder named test_2 and copy its contents to test, contents are locked
    current_dir_3.push("test_2");
    let test_dir_string = current_dir_3.to_string_lossy().to_string();
    fs::create_dir_all(&test_dir_string).unwrap();

    //we are trying to lock the file and the folder
    File::create("test_2\\foo.txt").unwrap();
    let mut folder_perms = fs::metadata("test_2").unwrap().permissions();
    folder_perms.set_readonly(true);
    fs::set_permissions("test_2", folder_perms).unwrap();

    let mut file_perms = fs::metadata("test_2\\foo.txt").unwrap().permissions();
    file_perms.set_readonly(true);
    fs::set_permissions("test_2\\foo.txt", file_perms).unwrap();

    //string for directory
    current_dir_2.push("test");
    let dest_dir_string = &current_dir_2.to_string_lossy().to_string();
    println!("{:?}", dest_dir_string);

    fs::create_dir_all(dest_dir_string).unwrap();

    //string for file
    current_dir.push("foo.txt");

    let origin_dir_string = &current_dir.to_string_lossy().to_string();
    println!("{:?}", origin_dir_string);

    let result = copy_snapshot_command(
        test_dir_string.to_owned(),
        dest_dir_string.to_owned().to_owned(),
    );

    assert!(result.len() > 0);
    assert_ne!(result, "sucess");

    //we do this to perform cleanup
    let mut folder_perms = fs::metadata("test_2").unwrap().permissions();
    folder_perms.set_readonly(false);
    fs::set_permissions("test_2", folder_perms).unwrap();

    let mut file_perms = fs::metadata("test_2\\foo.txt").unwrap().permissions();
    file_perms.set_readonly(false);
    fs::set_permissions("test_2\\foo.txt", file_perms).unwrap();

    //cleanup
    std::thread::sleep(time::Duration::from_secs(5));
    fs::remove_dir_all(&dest_dir_string).unwrap();
    fs::remove_dir_all(&test_dir_string).unwrap();
}

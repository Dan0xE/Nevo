use std::{
    fs::{self, File},
    path::Path,
    time::Duration,
};

use crate::utils::launch_game::launch_game;

#[test]
fn launch_game_failure_no_bat_file() {
    std::thread::sleep(Duration::from_secs(10));
    if Path::new("args.bat").exists() {
        fs::remove_file("args.bat").unwrap();
    }

    let result = launch_game();
    assert!(result.is_err());
    std::thread::sleep(Duration::from_secs(2))
}

#[test]
fn launch_game_sucess() {
    std::thread::sleep(Duration::from_secs(2));
    File::create("args.bat").unwrap();
    std::thread::sleep(Duration::from_secs(2));
    fs::write("args.bat", "echo hello").unwrap();

    let result = launch_game();
    assert!(result.is_ok());
}

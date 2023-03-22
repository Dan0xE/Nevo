// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use commands::{
    copy_snapshot_command::{__cmd__copy_snapshot_command, copy_snapshot_command},
    generate_args_command::{__cmd__generate_args_command, generate_args_command},
    lauch_game_command::{__cmd__launch_game_command, launch_game_command},
    read_args_command::{__cmd__read_args_command, read_args_command},
    write_args_command::{__cmd__write_args_command, write_args_command},
};

mod utils {
    pub(crate) mod argument_wrapper;
    pub(crate) mod copy_snapshot;
    pub(crate) mod create_shortcut;
    pub(crate) mod generate_args;
    pub(crate) mod is_minecraft_running;
    pub(crate) mod launch_game;
    pub(crate) mod read_arguments;
    pub(crate) mod write_arguments;
}

mod commands {
    pub(crate) mod copy_snapshot_command;
    pub(crate) mod generate_args_command;
    pub(crate) mod lauch_game_command;
    pub(crate) mod read_args_command;
    pub(crate) mod write_args_command;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            write_args_command,
            read_args_command,
            copy_snapshot_command,
            launch_game_command,
            generate_args_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

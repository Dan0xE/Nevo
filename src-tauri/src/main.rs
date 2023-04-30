// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use commands::rewrite_args_command::{__cmd__rewrite_args_command, rewrite_args_command};

#[cfg(test)]
mod tests {
    mod copy_snapshot_test;
    mod generate_args_test;
    mod launch_game_test;
    mod read_argument_test;
    mod write_args_test;
}

mod utils {
    pub(crate) mod argument_wrapper;
    pub(crate) mod copy_snapshot;
    pub(crate) mod create_shortcut;
    pub(crate) mod generate_args;
    pub(crate) mod is_minecraft_running;
    pub(crate) mod launch_game;
    pub(crate) mod read_arguments;
    pub(crate) mod rewrite_args;
    pub(crate) mod shift_array;
    pub(crate) mod write_arguments;
}

mod commands {

    pub(crate) mod rewrite_args_command;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![rewrite_args_command])
        .run(tauri::generate_context!())
        .expect("error while running nevo");
}

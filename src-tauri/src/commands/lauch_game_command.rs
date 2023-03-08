use crate::utils::launch_game::launch_game;

#[tauri::command]
pub(crate) fn launch_game_command() {
    launch_game()
}

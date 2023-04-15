use crate::utils::launch_game::launch_game;

#[tauri::command]
pub(crate) fn launch_game_command() -> String {
    match launch_game() {
        Ok(_) => "sucess".to_string(),
        Err(e) => format!("Could not launch the game: {:?}", e.to_string()),
    }
}

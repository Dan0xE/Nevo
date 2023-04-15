use crate::utils::launch_game::launch_game;

#[tauri::command]
pub(crate) fn launch_game_command() -> Result<String, String> {
    match launch_game() {
        Ok(_) => Ok("sucess".to_string()),
        Err(e) => Err(format!("Could not launch the game: {:?}", e.to_string())),
    }
}

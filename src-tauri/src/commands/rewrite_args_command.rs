use crate::utils::rewrite_args::rewrite_args;

#[tauri::command]
pub(crate) fn rewrite_args_command(username: String, path: String) -> Result<String, String> {
    match rewrite_args(username, path) {
        Ok(_) => {
            println!("Changed Args");
            return Ok::<String, String>("sucess".to_string());
        }
        Err(e) => {
            println!("Something went wrong: {:?}", e);
            return Err::<String, String>(e.to_string());
        }
    };
}

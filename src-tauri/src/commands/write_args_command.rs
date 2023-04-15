use crate::utils::write_arguments::write_args;

#[tauri::command]
pub(crate) fn write_args_command(args: String) -> Result<String, String> {
    match write_args(args) {
        Ok(_) => {
            println!("Wrote Arguments");
            return Ok::<String, String>("sucess".to_string());
        }
        Err(e) => {
            println!("Failed to write: {:?}", e);
            return Err::<String, String>(e.to_string());
        }
    };
}

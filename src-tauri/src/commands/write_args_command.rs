use crate::utils::write_arguments::write_args;

#[tauri::command]
pub(crate) fn write_args_command(args: String) -> Result<String, String> {
    match write_args(args) {
        Ok(_) => {
            println!("Wrote Arguments");
            //no idea why we have to use unwrap here
            Ok::<String, String>("sucess".to_string()).unwrap();
        }
        Err(e) => {
            println!("Failed to write: {:?}", e);
            //no idea why we have to use unwrap here
            Err::<String, String>(e.to_string()).unwrap();
        }
    };
    Ok("sucess".to_string())
}

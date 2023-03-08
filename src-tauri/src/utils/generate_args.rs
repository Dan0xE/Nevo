use powershell_script::run;
use std::env::current_dir;

use super::is_minecraft_running::is_minecraft_running;

pub(crate) fn generate_args() -> bool {
    let current_dir = current_dir().unwrap_or_default();
    let file_path = current_dir.join("p.ps1");
    if is_minecraft_running() {
        match run(file_path.into_os_string().into_string().unwrap().as_str()) {
            Ok(output) => {
                println!("{}", output);
                true
            }
            Err(e) => {
                println!("Error while generating args: {}", e);
                false
            }
        }
    } else {
        println!("Please start minecraft");
        false
    }
}

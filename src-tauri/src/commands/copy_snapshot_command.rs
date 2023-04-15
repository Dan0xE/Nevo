use crate::utils::copy_snapshot::copy_snapshot;

#[tauri::command]
pub(crate) fn copy_snapshot_command(origin: String, destination: String) -> String {
    match copy_snapshot(&origin, &destination) {
        Ok(_) => {
            println!("Copied Snapshot");
            return "sucess".to_string();
        }
        Err(e) => {
            println!("Failed to copy: {:?}", e);
            return e.to_string();
        }
    };
}

use crate::utils::copy_snapshot::copy_snapshot;

#[tauri::command]
pub(crate) fn copy_snapshot_command(origin: String, destination: String) -> Result<String, String> {
    match copy_snapshot(&origin, &destination) {
        Ok(_) => Ok::<String, String>("sucess".to_string()),
        Err(e) => Err::<String, String>(e.to_string()),
    }
}

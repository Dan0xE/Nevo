use crate::utils::copy_snapshot::copy_snapshot;

//TODO doesnt really return an indicator of failure
#[tauri::command]
pub(crate) fn copy_snapshot_command(origin: String, destination: String) -> bool {
    match copy_snapshot(&origin, &destination) {
        Ok(_) => {
            println!("Copied Snapshot");
            return true;
        }
        Err(e) => {
            println!("Failed to copy: {:?}", e);
            return false;
        }
    };
}

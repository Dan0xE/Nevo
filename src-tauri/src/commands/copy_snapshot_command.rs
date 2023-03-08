use crate::utils::copy_snapshot::copy_snapshot;

#[tauri::command]
pub(crate) fn copy_snapshot_command(origin: String, destination: String) -> bool {
    let result = copy_snapshot(origin, destination);
    result
}

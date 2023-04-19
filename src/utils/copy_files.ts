import { invoke } from "@tauri-apps/api";

export function copy_files(old_path: string, new_path: string) {
  invoke("copy_snapshot_command", {
    origin: old_path,
    destination: new_path,
  }).catch((e) => {
    console.error(`Failed to copy files:\n${e}`);
    alert(`Failed to copy files:\n${e}`);
  });
}

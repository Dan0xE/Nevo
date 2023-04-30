import { invoke } from "@tauri-apps/api";
import log from "./log";

export function copy_files(old_path: string, new_path: string) {
  invoke("copy_snapshot_command", {
    origin: old_path,
    destination: new_path,
  }).catch((e) => {
    log(`Failed to copy files: ${e}`, "error");
  });
}

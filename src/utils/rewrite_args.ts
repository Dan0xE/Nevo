import { invoke } from "@tauri-apps/api";
import { shiftArrary } from "./shift_array";
import { copy_files } from "./copy_files";

export default function rewrite_args(username: string, path: string) {
  invoke("generate_args_command")
    .then(() => {
      return invoke("read_args_command");
    })
    .then((res) => {
      let args: string[] = shiftArrary(res as string[], 2);

      let newVersion = false;
      let version = args.findIndex((e) => e === "--version");
      if (parseFloat(args[version + 1].replace(/\./g, "")) < 1181) {
        newVersion = true;
      }

      for (let i = 0; i < args.length; i++) {
        if (!newVersion && args[i].includes("-Djava.library.path")) {
          let old = args[i];
          args[i] = "-Djava.library.path=" + path;
          if (path) {
            copy_files(old.substring(20), path);
          }
        } else if (args[i].includes("--username")) {
          if (username) {
            args[i + 1] = username;
          } else {
            alert("No username specified");
          }
        }
      }
      return invoke("write_args_command", {
        args: args.join(" "),
        path: path,
      });
    })
    .then(() => {
      return invoke("launch_game_command");
    })
    .catch((error) => {
      console.error(`Error:\n${error}`);
      alert(`Error:\n${error}`);
    });
}

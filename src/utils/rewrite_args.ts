import { invoke } from "@tauri-apps/api";

function copy_files(old_path: string, new_path: string) {
  invoke("copy_snapshot_command", {
    origin: old_path,
    destination: new_path,
  })
    .then((res) => {
      console.log("Copied files");
      return res;
    })
    .catch((e) => {
      console.error("Failed to copy files: ", e);
      alert(`Failed to copy files`);
    });
}

export default function rewrite_args(username: string, path: string) {
  invoke("generate_args_command").then((res) => {
    if (res) {
      //@ts-ignore
      invoke("read_args_command").then((res: string[]) => {
        let args: string[] = res;
        args.shift();
        args.shift();
        console.log(`Argument length is: ${args.length}`);

        for (let i = 0; i < args.length; i++) {
          if (args[i].includes("-Djava.library.path")) {
            let old = args[i];
            args[i] = "-Djava.library.path=" + path;
            let new_path_log = args[i];
            console.log(args[i]);
            console.log(`Original Path before modification is: ${old}`);
            console.log(`New Path is ${new_path_log}`);

            old = old.substring(20);

            copy_files(old, path);
            break;
          }
        }

        for (let i = 0; i < args.length; i++) {
          if (args[i].includes("--username")) {
            if (username) {
              args[i + 1] = username;
              console.log(args[i + 1]);
            } else {
              alert("No username specified");
            }
          }
        }

        let store = args.join(" ");

        console.log(store);
        if (path) {
          console.log(path);
          invoke("write_args_command", {
            args: store,
            path: path,
          }).then(() => {
            invoke("launch_game_command");
          });
        } else {
          alert("No path specified");
        }
      });
    } else {
      console.error("Failed to generate args");
      alert("failed to generate args, maybe you forgot to run minecraft?");
    }
  });
}

import { invoke } from "@tauri-apps/api";

function shiftArrary(arr: string[], count: number): string[] {
  for (let i = 0; i < count; i++) {
    arr.shift();
  }
  return arr;
}

function copy_files(old_path: string, new_path: string) {
  invoke("copy_snapshot_command", {
    origin: old_path,
    destination: new_path,
  })
    .then((res) => {
      if (res !== "sucess") {
        console.error(`Failed to copy files:\n${res}`);
        alert(`Failed to copy files:\n${res}`);
      }
    })
    .catch((e) => {
      console.error(`Failed to copy files:\n${e}`);
      alert(`Failed to copy files:\n${e}`);
    });
}

export default function rewrite_args(username: string, path: string) {
  invoke("generate_args_command").then((res) => {
    if (res !== "sucess") {
      console.error(`Failed to generate args:\n${res}`);
      alert(`Failed to generate args:\n${res}`);
      return;
    }
    invoke("read_args_command")
      .then((res) => {
        let args: string[] = shiftArrary(res as string[], 2);
        console.log(args);
        if (args.length < 3) {
          console.error(`Failed to read args.txt\n${res}`);
          alert(`Failed to read args.txt\n${res}`);
          return;
        }

        let newVersion = false;

        for (let i = 0; i < args.length; i++) {
          if (args[i].includes("--version")) {
            let versionString = args[i + 1];
            console.log(versionString);
            let parseVersion = versionString.replace(/\./g, "");
            console.log(parseVersion);
            let version = parseFloat(parseVersion);

            if (version > 1.181) {
              newVersion = true;
            }
          }
        }

        if (!newVersion) {
          for (let i = 0; i < args.length; i++) {
            if (args[i].includes("-Djava.library.path")) {
              let old = args[i];
              args[i] = "-Djava.library.path=" + path;

              old = old.substring(20);

              copy_files(old, path);
            }
          }
        }

        for (let i = 0; i < args.length; i++) {
          if (args[i].includes("--username")) {
            if (username) {
              args[i + 1] = username;
            } else {
              console.error(`No username specified`);
              alert("No username specified");
            }
          }
        }

        let store = args.join(" ");

        if (path) {
          invoke("write_args_command", {
            args: store,
            path: path,
          })
            .then((res) => {
              if (res !== "sucess") {
                console.error(`Failed to write args:\n${res}`);
                alert(`Failed to write args:\n${res}`);
                return;
              }
              console;
              invoke("launch_game_command").then((res) => {
                if (res !== "sucess") {
                  console.error(`Failed to launch game:\n${res}`);
                  alert(`Failed to launch game:\n${res}`);
                  return;
                }
              });
            })
            .catch((e) => {
              alert(`Error while launching the game:\n${e}`);
              console.error(`Error while launching game\n${e}`);
            });
        } else {
          console.error(`No path specified`);
          alert("No path specified");
        }
      })
      .catch((e) => {
        console.error(`Failed to read args:\n${e}`);
        alert(`Failed to read args:\n${e}`);
      });
  });
}

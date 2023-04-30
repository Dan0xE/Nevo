import { useState } from "react";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api";

function App() {
  const [name, setName] = useState<string>("");
  const [path, setPath] = useState<string>("");

  function handle_submit() {
    let profileName = name.replace(/\s/g, "_");

    if (path.length > 255 || path.includes(" ")) {
      alert("invalid path\npath must not contain whitespaces");
      return;
    } else {
      invoke("rewrite_args_command", {
        username: profileName,
        path: path,
      }).catch((e) => alert(e));
    }
  }

  return (
    <div className="container">
      <p>Nevo</p>
      <div className="row">
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a Profile Name"
        />
        <button
          type="submit"
          onClick={(e) => {
            e.preventDefault();
            open({
              directory: true,
            }).then((res) => {
              if (res) {
                setPath(res as string);
              }
            });
          }}
        >
          Select Path (optional)
        </button>
        <button
          onClick={(e) => {
            e.preventDefault();
            handle_submit();
          }}
        >
          Launch Game
        </button>
      </div>
    </div>
  );
}

export default App;

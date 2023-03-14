import { useState } from "react";
import { open } from "@tauri-apps/api/dialog";
import rewrite_args from "./utils/rewrite_args";

function App() {
  const [name, setName] = useState<string>("");
  const [path, setPath] = useState<string>("");

  function handle_submit() {
    let profile = name.replace(/\s/g, "_");

    if (path.length > 255 || path.includes(" ")) {
      alert("Path is to long and cannot contain whitespaces");
      return;
    } else {
      rewrite_args(profile, path);
    }

    console.log(path, name);
  }

  return (
    <div className="container">
      <p>Snapshot Grabber</p>

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
          Select Path
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

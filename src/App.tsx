import { useState } from "react";
import { open } from "@tauri-apps/api/dialog";
import rewrite_args from "./utils/rewrite_args";
import log from "./utils/log";

function App() {
  const [name, setName] = useState<string>("");
  const [path, setPath] = useState<string>("");

  function handle_submit() {
    let profile = name.replace(/\s/g, "_");

    if (path.length > 255 || path.includes(" ")) {
      log("invalid path", "error");
      return;
    } else {
      rewrite_args(profile, path);
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

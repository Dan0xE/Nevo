import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";
import isTauriWindow from "./utils/is_tauri_windows";
import NoTauriWindow from "./pages/NoTauriWindow";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    {isTauriWindow() ? <App /> : <NoTauriWindow />}
  </React.StrictMode>
);

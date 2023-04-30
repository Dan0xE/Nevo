declare global {
  interface Window {
    __TAURI__: any;
  }
}

export default function isTauriWindow(): boolean {
  return window.__TAURI__;
}

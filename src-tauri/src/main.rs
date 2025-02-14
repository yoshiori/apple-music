// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

fn main() {
    tauri::Builder::default()
    .setup(|app| {
        let window = app.get_webview_window("main").unwrap();
        let _ = window.eval("window.location.replace('https://beta.music.apple.com/')");
        Ok(())
      })
      .run(tauri::generate_context!())
      .expect("error running tauri app");
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, RwLock};

use notify::PollWatcher;
use tauri::api::path::config_dir;
use tauri::Manager;

use better_poe::error::Result;
use better_poe::parser::create_file_watcher;
use better_poe::types::Settings;

pub const NAME: &str = env!("CARGO_PKG_NAME");

type Watcher = Arc<RwLock<Option<PollWatcher>>>;

#[tauri::command]
fn file_watcher(app_handle: tauri::AppHandle, path: &str) -> Result<()> {
  let state: tauri::State<'_, Watcher> = app_handle.state();

  let app_handle = app_handle.clone();
  let watcher = create_file_watcher(path, move |new_text| {
    app_handle.emit_all("test", new_text).unwrap();
  })?;

  *state.write().unwrap() = Some(watcher);

  Ok(())
}

#[tauri::command]
fn load_settings<'a>() -> Settings<'a> {
  config_dir()
    .map(|dir| dir.join(NAME))
    .and_then(|dir| std::fs::create_dir_all(&dir).ok().map(|_| dir))
    .map(|dir| dir.join("settings.json"))
    .filter(|file| file.exists())
    .and_then(|file| std::fs::read_to_string(file).ok())
    .and_then(|str| serde_json::from_str(&str).ok())
    .unwrap_or_default()
}

#[tauri::command]
fn save_settings(settings: Settings<'_>) {
  let _ = config_dir()
    .map(|dir| dir.join(NAME))
    .and_then(|dir| std::fs::create_dir_all(&dir).ok().map(|_| dir))
    .map(|dir| dir.join("settings.json"))
    .and_then(|file| {
      let str = serde_json::to_string_pretty(&settings).ok()?;

      std::fs::write(file, str).ok()
    });
}

fn main() {
  tauri::Builder::default()
    .manage(Watcher::default())
    .invoke_handler(tauri::generate_handler![
      file_watcher,
      load_settings,
      save_settings
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

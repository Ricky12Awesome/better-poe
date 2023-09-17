// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, RwLock};

use notify::PollWatcher;
use tauri::api::path::{cache_dir, config_dir};
use tauri::Manager;

use better_poe::error::Result;
use better_poe::misc::{load_file, save_file};
use better_poe::parser::create_file_watcher;
use better_poe::types::Settings;
use better_poe::NAME;

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
  load_file(config_dir().map(|dir| dir.join(NAME)), "settings.json").unwrap_or_default()
}

#[tauri::command]
fn save_settings(settings: Settings<'_>) {
  let _ = save_file(
    config_dir().map(|dir| dir.join(NAME)),
    "settings.json",
    settings,
  );
}

#[tauri::command]
fn load_state() -> better_poe::types::State {
  load_file(cache_dir().map(|dir| dir.join(NAME)), "state.json").unwrap_or_default()
}

#[tauri::command]
fn save_state(state: better_poe::types::State) {
  let _ = save_file(cache_dir().map(|dir| dir.join(NAME)), "state.json", state);
}

fn main() {
  tauri::Builder::default()
    .manage(Watcher::default())
    .invoke_handler(tauri::generate_handler![
      file_watcher,
      load_settings,
      save_settings,
      load_state,
      save_state,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

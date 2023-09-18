// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use notify::PollWatcher;
use std::sync::{Arc, RwLock};
use tauri::Manager;

use better_poe::error::Result;
use better_poe::parser::create_file_watcher;

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

fn main() {  
  tauri::Builder::default()
    .manage(Watcher::default())
    .invoke_handler(tauri::generate_handler![
      file_watcher,
      better_poe::api::auth::command::get_token,
      better_poe::storage::settings::load_settings,
      better_poe::storage::settings::save_settings,
      better_poe::storage::state::load_state,
      better_poe::storage::state::save_state,
      better_poe::storage::token::load_token,
      better_poe::storage::token::save_token,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

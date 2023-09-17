// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, RwLock};

use notify::PollWatcher;
use oauth2::TokenResponse;
use reqwest::Url;
use tauri::api::path::{cache_dir, config_dir};
use tauri::Manager;

use better_poe::error::Result;
use better_poe::misc::{load_file, save_file};
use better_poe::NAME;
use better_poe::parser::create_file_watcher;
use better_poe::types::{Settings, Token};

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

fn _load_settings<'a>() -> Result<Settings<'a>> {
  load_file(config_dir().map(|dir| dir.join(NAME)), "settings.json")
}

#[tauri::command]
fn load_settings<'a>() -> Settings<'a> {
  _load_settings().unwrap_or_default()
}

fn _save_settings(settings: Settings<'_>) -> Result<()> {
  save_file(
    config_dir().map(|dir| dir.join(NAME)),
    "settings.json",
    settings,
  )
}

#[tauri::command]
fn save_settings(settings: Settings<'_>) {
  let _ = _save_settings(settings);
}

#[tauri::command]
fn load_state() -> better_poe::types::State {
  load_file(cache_dir().map(|dir| dir.join(NAME)), "state.json").unwrap_or_default()
}

#[tauri::command]
fn save_state(state: better_poe::types::State) {
  let _ = save_file(cache_dir().map(|dir| dir.join(NAME)), "state.json", state);
}

#[tauri::command]
fn get_token<'a>(app_handle: tauri::AppHandle) -> Result<Token<'a>> {
  let open_url = |url: Url| app_handle.shell_scope().open(url.as_ref(), None);
  let token = better_poe::auth::get_token(open_url)?;
  let token = Token {
    access_token: token.access_token().secret().clone().into(),
    expires_in: token.expires_in().unwrap_or_default().as_secs(),
  };

  Ok(token)
}

fn main() {
  tauri::Builder::default()
    .manage(Watcher::default())
    .invoke_handler(tauri::generate_handler![
      file_watcher,
      get_token,
      load_settings,
      save_settings,
      load_state,
      save_state,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

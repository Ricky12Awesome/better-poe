use tauri::api::path::cache_dir;

use crate::api::auth::{Deserialize, Serialize, typeshare};
use crate::load_file_command;

#[typeshare]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct State {
  pub last_page: i32,
}

load_file_command!(load_state, save_state, state: State, cache_dir(), "state.json");

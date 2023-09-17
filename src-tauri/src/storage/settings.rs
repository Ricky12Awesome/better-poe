use crate::api::auth::{typeshare, Deserialize, Serialize};
use crate::load_file_command;
use std::borrow::Cow;
use std::path::PathBuf;
use tauri::api::path::config_dir;

#[typeshare]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Settings<'a> {
  #[typeshare(serialized_as = "Option<string>")]
  pub log_file: Option<PathBuf>,
  #[typeshare(serialized_as = "Option<string>")]
  pub theme: Option<Cow<'a, str>>,
}

load_file_command!(load_settings<'a>, save_settings, settings: Settings<'a>, config_dir(), "settings.json");

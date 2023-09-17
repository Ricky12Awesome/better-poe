use std::borrow::Cow;

use tauri::api::path::cache_dir;

use crate::api::auth::{typeshare, Deserialize, Serialize};
use crate::load_file_command;

#[typeshare]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Token<'a> {
  #[typeshare(serialized_as = "string")]
  pub access_token: Cow<'a, str>,
  #[typeshare(serialized_as = "string")]
  pub created: chrono::NaiveDateTime,
  #[typeshare(serialized_as = "string")]
  pub expires: chrono::NaiveDateTime,
}

load_file_command!(load_token<'a>, save_token, token: Token<'a>, cache_dir(), "token.json");

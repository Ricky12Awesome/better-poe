use std::borrow::Cow;
use chrono::Local;

use tauri::api::path::cache_dir;
use validator::{Validate, ValidationError};

use crate::api::auth::{typeshare, Deserialize, Serialize};
use crate::error::{Result, Error};
use crate::load_file_command;

#[typeshare]
#[derive(Debug, Validate, Clone, Serialize, Deserialize)]
pub struct Token<'a> {
  #[typeshare(serialized_as = "string")]
  #[validate(length(min = 1))]
  pub access_token: Cow<'a, str>,
  #[typeshare(serialized_as = "string")]
  pub created: chrono::NaiveDateTime,
  #[typeshare(serialized_as = "string")]
  pub expires: chrono::NaiveDateTime,
}

impl Token<'_> {
  pub fn to_validated(self) -> Result<Self> {
    self.validate()?;

    let now = Local::now().naive_local();

    if self.expires <= now {
      return Err(Error::ValidationError(ValidationError::new("expired")))
    }

    Ok(self)
  }
}

load_file_command!(load_token<'a>, save_token, token: Token<'a>, cache_dir(), "token.json", and_then(|token: Token<'a>| token.to_validated()));

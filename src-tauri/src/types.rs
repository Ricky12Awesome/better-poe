use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::path::PathBuf;
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Token<'a> {
  #[typeshare(serialized_as = "string")]
  pub access_token: Cow<'a, str>,
  #[typeshare(serialized_as = "number")]
  pub expires_in: u64,
}
#[typeshare]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
  pub last_page: i32,
}

#[typeshare]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Settings<'a> {
  #[typeshare(serialized_as = "Option<string>")]
  pub log_file: Option<PathBuf>,
  #[typeshare(serialized_as = "Option<string>")]
  pub theme: Option<Cow<'a, str>>,
  pub token: Option<Token<'a>>,
}

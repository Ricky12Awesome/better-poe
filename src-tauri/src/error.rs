use serde::{Serialize, Serializer};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Infallible(#[from] std::convert::Infallible),
  #[error(transparent)]
  ParseError(#[from] oauth2::url::ParseError),
  #[error(transparent)]
  SerdeJson(#[from] serde_json::Error),
  #[error("Failed to get authorization code")]
  FailedToGetAuthorizationCode,
  #[error(transparent)]
  Any(#[from] anyhow::Error),
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}

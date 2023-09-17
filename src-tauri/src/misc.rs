use std::path::{Path, PathBuf};

use crate::error::Result;

pub fn load_file<T>(base_dir: Option<PathBuf>, name: impl AsRef<Path>) -> Result<T>
where
  T: serde::de::DeserializeOwned,
{
  let dir = base_dir.unwrap_or_default();

  std::fs::create_dir_all(&dir)?;

  let file = dir.join(name);
  let str = std::fs::read_to_string(file)?;
  let t = serde_json::from_str(&str)?;

  Ok(t)
}

pub fn save_file<T>(base_dir: Option<PathBuf>, name: impl AsRef<Path>, data: T) -> Result<()>
where
  T: serde::ser::Serialize,
{
  let dir = base_dir.unwrap_or_default();

  std::fs::create_dir_all(&dir)?;

  let file = dir.join(name);
  let str = serde_json::to_string_pretty(&data)?;

  std::fs::write(file, str)?;

  Ok(())
}

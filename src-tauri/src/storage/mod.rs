use std::path::{Path, PathBuf};

use crate::error;

pub mod settings;
pub mod state;
pub mod token;

#[macro_export]
macro_rules! load_file_command {
  (
    no_app_name: $lf:ident$(<$l:lifetime>)?, $sf:ident, $tn:ident: $t:ty, $dir:expr, $name:expr
    $(,$($ff:ident($map:expr)).+)?
  ) => {
    #[tauri::command]
    pub fn $lf$(<$l>)?() -> $crate::error::Result<$t> {
      $crate::storage::load_file($dir, $name)$($(.$ff($map))+)?
    }

    #[tauri::command]
    pub fn $sf$(<$l>)?($tn: $t) -> $crate::error::Result<()> {
      $crate::storage::save_file(
        $dir,
        $name,
        $tn,
      )
    }
  };
  (
    $lf:ident$(<$l:lifetime>)?, $sf:ident, $tn:ident: $t:ty, $dir:expr, $name:expr
    $(,$($ff:ident($map:expr)).+)?
  ) => {
    load_file_command!(
      no_app_name:
      $lf$(<$l>)?,
      $sf,
      $tn: $t,
      ($dir).map(|dir| dir.join($crate::NAME)),
      $name
      $(,$($ff($map)).+)?
    );
  }
}

pub fn load_file<T>(base_dir: Option<PathBuf>, name: impl AsRef<Path>) -> error::Result<T>
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

pub fn save_file<T>(base_dir: Option<PathBuf>, name: impl AsRef<Path>, data: T) -> error::Result<()>
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

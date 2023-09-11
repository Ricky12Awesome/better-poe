//! This is really only for testing purposes
//!
use std::io::{Read, stdin};
use std::sync::{Arc, RwLock};
use notify::PollWatcher;

use better_poe::parser::create_file_watcher;

fn main() -> anyhow::Result<()> {
  // Can't be bothered setting up env for this
  let file = include_str!("secret/path.txt");
  let data = Arc::new(RwLock::new(String::new()));

  let watcher_handle = data.clone();
  let _watcher: PollWatcher = create_file_watcher(file, move |new_text| {
    print!("{new_text}");
    watcher_handle.write().unwrap().push_str(new_text);
  })?;

  // wait until some input
  stdin().bytes().next();

  std::fs::write("examples/secret/output.txt", data.read().unwrap().as_bytes())?;

  Ok(())
}

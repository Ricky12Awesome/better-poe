use std::process::Command;

fn main() {
  let _ = Command::new("typeshare")
    .args([".", "--lang", "typescript", "--output-file", "../src/lib/gen/types.ts"])
    .spawn();

  tauri_build::build()
}

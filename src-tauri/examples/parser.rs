//! This is mainly for testing purposes
//!

use better_poe::parser::{parse_lines, ParsedLine};

const LINES: &str = include_str!("secret/poe log output.txt");

fn main() -> anyhow::Result<()> {
  for line in parse_lines(LINES) {
    if let ParsedLine::SomeoneJoinedArea(line) = line {
      println!("{line:?}");
    }
  }

  Ok(())
}

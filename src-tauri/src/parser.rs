use std::fs::OpenOptions;
use std::io::{read_to_string, Seek, SeekFrom};
use std::path::Path;
use std::time::Duration;

use chrono::NaiveDateTime;
use notify::{Config, Event, EventKind, RecursiveMode, Watcher};

pub fn create_file_watcher<W, P, F>(file: P, callback: F) -> anyhow::Result<W>
where
  W: Watcher,
  P: AsRef<Path>,
  F: Fn(&str) + Send + 'static,
{
  let mut reader = OpenOptions::new().read(true).open(file.as_ref())?;

  reader.seek(SeekFrom::End(0))?;

  let event_handler = move |res: Result<Event, _>| {
    match res {
      Ok(event) => {
        if let EventKind::Modify(_) = event.kind {
          let str = read_to_string(&mut reader).unwrap();

          if str.is_empty() {
            return;
          }

          callback(&str);
        }
      }
      Err(err) => eprintln!("{err}"),
    };
  };

  let mut watcher = W::new(
    event_handler,
    Config::default().with_poll_interval(Duration::from_millis(1000)),
  )?;

  watcher.watch(file.as_ref(), RecursiveMode::NonRecursive)?;

  Ok(watcher)
}

#[derive(Debug, Clone)]
pub enum ParsedLine<'a> {
  EnteredHideout(EnteredHideout<'a>),
  SomeoneJoinedArea(SomeoneJoinedArea<'a>),
  Other,
}

#[derive(Debug, Clone)]
pub struct EnteredHideout<'a> {
  pub time: NaiveDateTime,
  pub name: &'a str,
}

#[derive(Debug, Clone)]
pub struct SomeoneJoinedArea<'a> {
  pub time: NaiveDateTime,
  pub name: &'a str,
}

pub fn parse_hideouts(text: &str) -> impl Iterator<Item = EnteredHideout> {
  text.lines().map(parse_line).filter_map(|line| match line {
    ParsedLine::EnteredHideout(object) => Some(object),
    _ => None,
  })
}

pub fn parse_lines(text: &str) -> impl Iterator<Item = ParsedLine> {
  text.lines().map(parse_line).filter_map(|line| match line {
    ParsedLine::Other => None,
    line => Some(line),
  })
}

fn find_joined_area(line: &str) -> Option<&str> {
  let text = " has joined the area";

  line
    .rfind(text)
    .and_then(|end| line.find(": ").map(|start| (start, end)))
    .map(|(start, end)| &line[start + 2..end])
}

fn find_hideout(line: &str) -> Option<&str> {
  let text = " You have entered ";

  line.rfind(text).map(|pos| &line[pos + text.len()..])
}

pub fn parse_line(line: &str) -> ParsedLine {
  if line.len() < 20 {
    return ParsedLine::Other;
  }

  let Ok(time) = NaiveDateTime::parse_from_str(&line[..19], "%Y/%m/%d %H:%M:%S") else {
    return ParsedLine::Other;
  };

  if let Some(name) = find_hideout(line) {
    return ParsedLine::EnteredHideout(EnteredHideout { time, name });
  }

  if let Some(name) = find_joined_area(line) {
    return ParsedLine::SomeoneJoinedArea(SomeoneJoinedArea { time, name });
  }

  ParsedLine::Other
}

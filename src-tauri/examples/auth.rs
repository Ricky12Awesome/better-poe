fn main() {
  let response = better_poe::auth::get_token(|url| println!("{url}")).unwrap();
  let token = serde_json::to_string_pretty(&response).unwrap();

  println!("{token}");

  std::fs::write("examples/secret/token.json", token).unwrap();
}

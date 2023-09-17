pub mod api;
pub mod error;
pub mod parser;
pub mod storage;

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTACT_EMAIL: &str = "betterpoe@skiff.com";
pub const CLIENT_ID: &str = "betterpoe";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GRANT_TYPE: &str = "authorization_code";
pub const API_URL: &str = "https://api.pathofexile.com";
pub const POE_URL: &str = "https://www.pathofexile.com";
pub const AUTH_URL: &str = const_format::concatcp!(POE_URL, "/oauth/authorize");
pub const TOKEN_URL: &str = const_format::concatcp!(POE_URL, "/oauth/token");
pub const AUTO_CLOSE_HTML: &str = include_str!("auto_close.html");
pub const REDIRECT_URL: &str = "http://localhost:8088";
pub const PROFILE_SCOPE: &str = "account:profile";
pub const LEAGUES_SCOPE: &str = "account:leagues";
pub const STASHES_SCOPE: &str = "account:stashes";
pub const CHARACTERS_SCOPE: &str = "account:characters";

//OAuth {$clientId}/{$version} (contact: {$contact}) ...
pub const USER_AGENT: &str = const_format::concatcp!(
  "OAuth ",
  CLIENT_ID,
  "/",
  VERSION,
  " ",
  "(contact: ",
  CONTACT_EMAIL,
  ")"
);

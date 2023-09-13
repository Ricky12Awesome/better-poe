use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::{
  AuthUrl, AuthorizationCode, ClientId, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenUrl,
};
use reqwest::Url;

use crate::error::{Error, Result};

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

pub fn get_authorization_code(state: CsrfToken) -> Result<String> {
  let server = tiny_http::Server::http("127.0.0.1:8088").unwrap();

  for request in server.incoming_requests() {
    let url = format!("http://{}{}", server.server_addr(), request.url());
    let url = oauth2::url::Url::parse(&url)?;
    let query = url.query_pairs().collect::<Vec<_>>();

    let query_state = query
      .iter()
      .find(|(key, _)| key == "state")
      .map(|(_, value)| value);

    let query_code = query
      .iter()
      .find(|(key, _)| key == "code")
      .map(|(_, value)| value);

    match (query_state, query_code) {
      (Some(query_state), Some(query_code)) if state.secret() == query_state => {
        request.respond(tiny_http::Response::from_data(AUTO_CLOSE_HTML))?;

        return Ok(query_code.to_string());
      }
      _ => {
        request.respond(tiny_http::Response::from_string("Invalid Query").with_status_code(422))?;
      }
    }
  }

  Ok(String::new())
}

pub fn get_token(callback: impl FnOnce(Url)) -> Result<BasicTokenResponse> {
  let client = BasicClient::new(
    ClientId::new(CLIENT_ID.to_string()),
    None,
    AuthUrl::new(AUTH_URL.to_string())?,
    Some(TokenUrl::new(TOKEN_URL.to_string())?),
  )
  .set_redirect_uri(RedirectUrl::new(REDIRECT_URL.to_string())?);

  let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
  let (auth_url, csrf_token) = client
    .authorize_url(CsrfToken::new_random)
    // Set the desired scopes.
    .add_scopes([
      Scope::new(PROFILE_SCOPE.to_string()),
      Scope::new(LEAGUES_SCOPE.to_string()),
      Scope::new(STASHES_SCOPE.to_string()),
      Scope::new(CHARACTERS_SCOPE.to_string()),
    ])
    .set_pkce_challenge(pkce_challenge)
    .url();

  callback(auth_url);

  let authorization_code = get_authorization_code(csrf_token)?;

  client
    .exchange_code(AuthorizationCode::new(authorization_code))
    .set_pkce_verifier(pkce_verifier)
    .request(oauth2::reqwest::http_client)
    .map_err(|_| Error::TokenError)
}

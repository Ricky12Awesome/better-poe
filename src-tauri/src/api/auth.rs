pub use std::borrow::Cow;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub use chrono::Local;
pub use oauth2::basic::{BasicClient, BasicTokenResponse};
pub use oauth2::{
  AuthUrl, AuthorizationCode, ClientId, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope,
  TokenResponse, TokenUrl,
};
pub use reqwest::Url;
pub use serde::{Deserialize, Serialize};
pub use tauri::Manager;
pub use typeshare::typeshare;

pub use crate::error::{Error, Result};
pub use crate::{
  AUTH_URL, AUTO_CLOSE_HTML, CHARACTERS_SCOPE, CLIENT_ID, LEAGUES_SCOPE, PROFILE_SCOPE,
  REDIRECT_URL, STASHES_SCOPE, TOKEN_URL,
};

pub fn get_authorization_code(state: CsrfToken) -> Result<String> {
  let server = tiny_http::Server::http("127.0.0.1:8088").unwrap();

  std::thread::scope(|s| {
    let thread = s.spawn(|| -> Result<String> {
      for request in server.incoming_requests() {
        let url = format!("{}{}", REDIRECT_URL, request.url());
        let url = Url::parse(&url)?;
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
            request
              .respond(tiny_http::Response::from_string("Invalid Query").with_status_code(422))?;
          }
        }
      }

      Err(Error::FailedToGetAuthorizationCode)
    });

    let start = Instant::now();

    while start.elapsed() <= Duration::from_secs(5 * 60) {
      if thread.is_finished() {
        return thread
          .join()
          .map_err(|_| Error::FailedToGetAuthorizationCode)?;
      }

      sleep(Duration::from_millis(100))
    }

    server.unblock();

    Err(Error::FailedToGetAuthorizationCode)
  })
}

pub fn get_token<F, E>(callback: F) -> Result<BasicTokenResponse>
where
  F: FnOnce(Url) -> Result<(), E>,
  E: Into<Error>,
{
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

  callback(auth_url).map_err(Into::into)?;

  let authorization_code = get_authorization_code(csrf_token)?;

  client
    .exchange_code(AuthorizationCode::new(authorization_code))
    .set_pkce_verifier(pkce_verifier)
    .request(oauth2::reqwest::http_client)
    .map_err(|_| Error::FailedToGetAuthorizationCode)
}

pub mod command {
  use crate::storage::token::Token;

  use super::get_token as _get_token;
  use super::*;

  #[tauri::command]
  pub async fn get_token(app_handle: tauri::AppHandle) -> Result<()> {
    tauri::async_runtime::spawn_blocking(move || {
      let open_url = |url: Url| app_handle.shell_scope().open(url.as_ref(), None);
      let token = _get_token(open_url)?;
      let access_token = token.access_token().secret().clone().into();
      let created = Local::now().naive_local();
      let mut expires = created;

      expires += token.expires_in().unwrap_or_default();
      expires -= Duration::from_secs(60 * 60);

      let token = Token {
        access_token,
        created,
        expires,
      };

      app_handle
        .emit_all("get_token_result", token)
        .map_err(Error::from)
    })
    .await
    .unwrap()
  }
}

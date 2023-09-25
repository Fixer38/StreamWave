use std::fmt::Error;
use oauth2::{
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    RedirectUrl,
    Scope
};
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use url::ParseError;

// TODO: Create the library based on an access token that's been retrieved outside of the library scope
// TODO: Create a client and pass to it an access token and create tests based on that
fn main() {

    let client =
        BasicClient::new(
            ClientId::new("".to_string()),
            Some(ClientSecret::new("".to_string())),
            AuthUrl::new("https://id.twitch.tv/oauth2/token".to_string())?,
            None,
        );

    let token_result = client.exchange_client_credentials().request(http_client)?;
    println!("Access token: {:?}", token_result)
}

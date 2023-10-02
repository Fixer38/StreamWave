use oauth2::{AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl};
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct OauthResponse {
    access_token: String,
    expires_in: i32,
    token_type: String
}

// TODO: Create the library based on an access token that's been retrieved outside of the library scope
// TODO: Create a client and pass to it an access token and create tests based on that
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = reqwest::Client::new();
    let resp = client.post("https://id.twitch.tv/oauth2/token")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body("client_id=8ximxajaf3jbcx9v39bmkuxsr5w904&client_secret=grant_type=client_credentials")
        .send()
        .await?;
    match resp.status() {
        reqwest::StatusCode::OK => {
            match resp.json::<OauthResponse>().await {
                Ok(parsed_response) => println!("Parsed response: {:?}", parsed_response),
                Err(_) => println!("Error in the parsing of a successful response"),
            };
        }
        _ => {
            println!("Error during the auth process: {:?}", resp.status())
        }
    }
    Ok(())
}

use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AccessToken,
    AuthUrl,
    ClientId,
    ClientSecret,
    Scope,
    TokenResponse,
    TokenUrl
};
use std::fs;
use std::env;

pub async fn get_token() -> String {
    return match env::var("VTAccessToken") {
        Ok(token) => token,
        Err(_) => update_token().await.secret().to_string()
    };
}

pub async fn update_token() -> AccessToken {
    println!("Updating token!");

    let clientid = match env::var("VTClient") {
        Ok(id) => id,
        Err(_) => fs::read_to_string("/run/secrets/VTClient").unwrap()
    };
    let clientsecret = match env::var("VTSecret") {
        Ok(secret) => secret,
        Err(_) => fs::read_to_string("/run/secrets/VTSecret").unwrap()
    };
    let token_url = TokenUrl::new("https://ext-api.vasttrafik.se/token".to_string())
        .expect("Invalid token url");
    
    let client = BasicClient::new(
        ClientId::new(clientid),
        Some(ClientSecret::new(clientsecret)),
        AuthUrl::new("https://ext-api.vasttrafik.se/token".to_string()).expect("Nonononono"),
        Some(token_url)
    );
    
    let token_result = client.exchange_client_credentials()
    .add_scope(Scope::new("app".to_string()))
    .request_async(async_http_client).await.expect("Error fetching token");
    let token = token_result.access_token().secret();

    env::set_var("VTAccessToken", token);
    return token_result.access_token().clone();
}

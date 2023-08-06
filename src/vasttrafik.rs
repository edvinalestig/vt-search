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

// use serde::{Serialize, Deserialize};
// use serde::de::DeserializeOwned;
use std::env;
// use reqwest::StatusCode;

// const BASE_URL: &str = "https://ext-api.vasttrafik.se/pr/v4";

// #[derive(Serialize, Deserialize, Debug)]
// struct APIResponse {
//     results: Vec<String>,
//     pagination: String,
//     links: Vec<String>
// }

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

// async fn request<ResponseModel>(url: &str) -> ResponseModel 
//         where ResponseModel: DeserializeOwned {
//     let token = match env::var("VTAccessToken") {
//         Ok(token) => token,
//         Err(_) => update_token().await.secret().to_string()
//     };
//     let client = reqwest::Client::new();
//     let response = client
//         .get(BASE_URL.to_owned() + url)
//         .header("AUTHORIZATION", "Bearer ".to_owned() + &token)
//         .send()
//         .await
//         .expect("Something went wrong");
//     let res = match response.status() {
//         StatusCode::OK => response.json::<ResponseModel>().await.unwrap(),
//         StatusCode::UNAUTHORIZED => {
//                 let token2 = update_token().await.secret().to_string();
//                 let response2 = client
//                     .get(BASE_URL.to_owned() + url)
//                     .header("AUTHORIZATION", "Bearer ".to_owned() + &token2)
//                     .send()
//                     .await
//                     .expect("Something went wrong");
//                 match response2.status() {
//                     StatusCode::OK => response2.json::<ResponseModel>().await.unwrap(),
//                     code => panic!("Could not get a response from VT: {:?}", code)
//                 }
//             },
//         code => panic!("Could not get a response from VT: {:?}", code)
//     };

//     // let text = response.text().await.unwrap();

//     return res;
// }

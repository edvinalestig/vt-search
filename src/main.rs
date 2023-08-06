pub mod vasttrafik;
use openapi::apis::{locations_api, configuration::Configuration};
use axum::{response::Html, routing::get, Router};
use vasttrafik::update_token;

async fn get_api_config() -> Configuration {
    let mut config = Configuration::new();
    let token: String = vasttrafik::get_token().await;
    config.oauth_access_token = Some(token);
    return config;
}

async fn home() -> Html<&'static str> {
    return Html("<a href='/test'>Hello world</a>");
}

async fn test() -> String {
    return testrun("brunnsparken").await;
}

async fn testrun(query: &str) -> String {
    let config = get_api_config().await;

    let results = match locations_api::locations_by_text_get(
        &config, query, 
        Some(vec![openapi::models::LocationByTextType::Stoparea]),
        None, None).await {
            Ok(res) => res.results.unwrap().unwrap(),
            Err(_) => {
                update_token().await;
                let config = get_api_config().await;
                locations_api::locations_by_text_get(
                    &config, 
                    "brunnsparken", 
                    Some(vec![openapi::models::LocationByTextType::Stoparea]),
                    None, 
                    None).await.unwrap().results.unwrap().unwrap()
            }
    };

    let first = &results[0].name;

    return first.clone();
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/test", get(test));

    axum::Server::bind(&"0.0.0.0:5000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

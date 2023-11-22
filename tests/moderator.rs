use dash_rs::request::account::LoginRequest;
use dash_rs::request::{CONTENT_TYPE, URL_FORM_ENCODED};
use dash_rs::request::moderator::{SuggestedFeatureScore, SuggestedStars, SuggestStarsRequest};

#[tokio::test]
async fn suggest_level() {
    dotenv::from_filename("test_env.env").expect("test_env.env file not found");

    let user_name = dotenv::var("GJ_ACCOUNT_USERNAME").unwrap();
    let password = dotenv::var("GJ_ACCOUNT_PASSWORD").unwrap();
    let client = reqwest::Client::new();

    let request = LoginRequest::default()
        .user_name(&user_name)
        .password(&password);

    let login_response = request.to_authenticated_user()
        .await
        .unwrap();

    let suggest_level_request = SuggestStarsRequest::new(login_response, 95524621)
        .feature(SuggestedFeatureScore::Featured)
        .stars(SuggestedStars::Ten);

    println!("{}", &suggest_level_request.to_string());
    let suggest_level_response = client.post(suggest_level_request.to_url())
        .body(suggest_level_request.to_string())
        .header(CONTENT_TYPE, URL_FORM_ENCODED)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    assert_eq!(suggest_level_response, "1")
}
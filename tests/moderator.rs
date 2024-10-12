use std::borrow::Cow;
use dash_rs::request::account::{AuthenticatedUser, LoginRequest};
use dash_rs::request::{CONTENT_TYPE, URL_FORM_ENCODED};
use dash_rs::request::moderator::{SuggestedFeatureScore, SuggestedStars, SuggestStarsRequest};

#[tokio::test]
async fn suggest_level() {
    let client = reqwest::Client::new();

    let login_response = AuthenticatedUser::new(
        "Ryder",
        57903,
        Cow::Borrowed("UmVkaXNuZU1FQXJFREdlTnRJQw==")
    );

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
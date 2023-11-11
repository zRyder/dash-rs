use dash_rs::{
    request::{
        account::LoginRequest,
        comment::{UploadCommentRequest, DeleteCommentRequest, CommentHistoryRequest, LevelCommentsRequest, ProfileCommentsRequest, SortMode},
    },
};
use dash_rs::response::{parse_get_gj_acccount_comments_response, parse_get_gj_comments_response};

const CONTENT_TYPE: &str = "Content-Type";
const URL_FORM_ENCODED: &str = "application/x-www-form-urlencoded";

#[tokio::test]
async fn get_level_comments() {
    let client = reqwest::Client::new();

    let request = LevelCommentsRequest::new(76298358)
        .page(0)
        .limit(5)
        .most_recent();

    let raw_response = client.post(request.to_url())
        .body(request.to_string())
        .header(CONTENT_TYPE, URL_FORM_ENCODED)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{}", &raw_response);

    let level_comments = parse_get_gj_comments_response(&raw_response).unwrap();

    assert_eq!(level_comments.len(), 5);
}


#[tokio::test]
async fn get_profile_comments() {
    let client = reqwest::Client::new();

    let request = ProfileCommentsRequest::new(57903)
        .page(0);

    let raw_response = client.post(request.to_url())
        .body(request.to_string())
        .header(CONTENT_TYPE, URL_FORM_ENCODED)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();


    let profile_comments = parse_get_gj_acccount_comments_response(&raw_response)
        .unwrap();

    assert_eq!(profile_comments.len(), 3);
}
#[tokio::test]
async fn get_comment_history() {
    let client = reqwest::Client::new();

    let comment_history_request = CommentHistoryRequest::new(17577805)
        .sort_mode(SortMode::Recent)
        .count(1)
        .page(0);

    let comment_history_response = client.post(comment_history_request.to_url())
        .body(comment_history_request.to_string())
        .header(CONTENT_TYPE, URL_FORM_ENCODED)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let comment_history_response = parse_get_gj_comments_response(&comment_history_response)
        .unwrap();
    println!("{:?}", &comment_history_response)
}

#[tokio::test]
async fn upload_comment() {
    dotenv::from_filename("test_env.env").expect("test_env.env file not found");

    let user_name = dotenv::var("GJ_ACCOUNT_USERNAME").unwrap();
    let password = dotenv::var("GJ_ACCOUNT_PASSWORD").unwrap();
    let client = reqwest::Client::new();

    let login_request = LoginRequest::default()
        .user_name(&user_name)
        .password(&password);

    let login_response = login_request.to_authenticated_user()
        .await
        .unwrap();

    let comment_upload_request = UploadCommentRequest::new(login_response, 76298358)
        .comment("More tests still ignore me")
        .percent(69);

    let response = client.post(comment_upload_request.to_url())
        .body(comment_upload_request.to_string())
        .header(CONTENT_TYPE, URL_FORM_ENCODED)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    assert!(!response.eq("-1"))
}

#[tokio::test]
async fn delete_comment() {
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

    let comment_history_request = CommentHistoryRequest::new(3713125)
        .sort_mode(SortMode::Recent)
        .count(1)
        .page(0);

    let comment_history_response = client.post(comment_history_request.to_url())
        .body(comment_history_request.to_string())
        .header(CONTENT_TYPE, URL_FORM_ENCODED)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let comment_history_response = parse_get_gj_comments_response(&comment_history_response)
        .unwrap();

    let comment_id = comment_history_response.get(0).unwrap().comment_id;

    let comment_delete_request = DeleteCommentRequest::new(login_response, 76298358, comment_id);

    let comment_delete_response = client.post(comment_delete_request.to_url())
        .body(comment_delete_request.to_string())
        .header(CONTENT_TYPE, URL_FORM_ENCODED)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    assert!(!comment_delete_response.eq("-1"))
}
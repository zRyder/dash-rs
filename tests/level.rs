use std::borrow::Cow;

use dash_rs::{
    model::{
        level::{DemonRating, Featured, LevelLength, LevelRating},
        song::MainSong,
        GameVersion,
    },
    Base64Decoded, Thunk,
};
use dash_rs::request::level::{LevelRequest, LevelsRequest};

#[tokio::test]
async fn download_gj_level_test() {
    let request = LevelRequest::new(72308725);

    let response_body =  request.get_response_body()
        .await
        .unwrap();

    let level = request.into_robtop(&response_body)
        .await
        .unwrap();

    println!("{:?}", level)
}

#[tokio::test]
async fn get_gj_levels_test() {
    let request = LevelsRequest::default()
        .search("The Nightmare")
        .page(0);

    let response_body =  request.get_response_body()
        .await
        .unwrap();

    println!("{}", &response_body);

    let levels = request.into_robtop(&response_body)
        .await
        .unwrap();

    println!("{:?}", levels.get(0))
}
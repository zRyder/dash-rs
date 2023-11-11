use dash_rs::{
    model::{
        level::LevelLength,
    },
};
use dash_rs::model::level::DemonRating::Extreme;
use dash_rs::model::level::LevelRating::Demon;
use dash_rs::model::level::Password::PasswordCopy;
use dash_rs::request::level::{LevelRequest, LevelsRequest};
use dash_rs::response::{parse_download_gj_level_response, parse_get_gj_levels_response};

const CONTENT_TYPE: &str = "Content-Type";
const URL_FORM_ENCODED: &str = "application/x-www-form-urlencoded";

#[tokio::test]
async fn download_gj_level_test() {
    let client = reqwest::Client::new();
    let request = LevelRequest::new(76298358);

    let raw_response = client.post(request.to_url())
        .body(request.to_string())
        .header(CONTENT_TYPE, URL_FORM_ENCODED)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let level = parse_download_gj_level_response(&raw_response)
        .unwrap();

    println!("{:?}", &level);

    assert_eq!(level.name, "Edooox Collab");
    assert_eq!(level.level_data.as_ref().unwrap().password, PasswordCopy(7678));
    assert_eq!(level.level_data.as_ref().unwrap().editor_time.unwrap(), 432);

}

#[tokio::test]
async fn get_gj_levels_test() {
    let client = reqwest::Client::new();
    let request = LevelsRequest::default()
        .search("Spectrum Rave")
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

    let levels = parse_get_gj_levels_response(&raw_response)
        .unwrap();

    let level = levels.get(0).unwrap();

    println!("{:?}", levels.get(0));

    assert_eq!(level.level_id, 72308725);
    assert_eq!(level.name, "Spectrum Rave");
    assert_eq!(level.creator.as_ref().unwrap().name, "Ryder");
    assert_eq!(level.main_song, None);
    assert_eq!(level.length, LevelLength::Long);
    assert_eq!(level.custom_song.as_ref().unwrap().song_id, 785444);
    assert_eq!(level.has_verified_coins, false);
    assert_eq!(level.is_epic, true);
    assert_eq!(level.difficulty, Demon(Extreme));
    assert_eq!(level.length, LevelLength::Long);
    assert!(level.level_data.as_ref().is_none());
}

#[tokio::test]
async fn get_gj_levels_should_use_main_song_test() {
    let client = reqwest::Client::new();
    let request = LevelsRequest::default()
        .search("The Nightmare")
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

    let levels = parse_get_gj_levels_response(&raw_response)
        .unwrap();

    let level = levels.get(0).unwrap();

    assert_eq!(level.level_id, 13519);
    assert_eq!(level.main_song.as_ref().unwrap().name, "Polargeist");
    assert!(level.custom_song.as_ref().is_none());
}
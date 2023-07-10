#![allow(unused_imports)]
use crate::{
    model::{
        level::{
            DemonRating, Objects, Password, Featured, LevelLength, LevelRating,
            local_level::LevelData,
            online_level::Level,
        },
        song::MainSong,
        GameVersion,
    },
    serde::{Base64Decoded, IndexedDeserializer, IndexedSerializer, Internal, RefThunk},
    DeError, HasRobtopFormat, SerError, Thunk,
};
use serde::{Deserialize, Serialize};
use std::{
    borrow::{Borrow, Cow},
    io::Write,
};

mod level_length {
    use crate::model::level::LevelLength;
    use serde::{de::Error, Deserialize, Deserializer, Serializer};

    // pub fn serialize<S>(to_serialize: &LevelLength, serializer: S) -> Result<S::Ok, S::Error>
    // where
    //     S: Serializer,
    // {
    //     match to_serialize {
    //         LevelLength::Unknown(unknown) => serializer.serialize_i32(*unknown),
    //         LevelLength::Tiny => serializer.serialize_str("0"),
    //         LevelLength::Short => serializer.serialize_str("1"),
    //         LevelLength::Medium => serializer.serialize_str("2"),
    //         LevelLength::Long => serializer.serialize_str("3"),
    //         LevelLength::ExtraLong => serializer.serialize_str("4"),
    //     }
    // }
    //
    // pub fn deserialize<'de, D>(deserializer: D) -> Result<LevelLength, D::Error>
    // where
    //     D: Deserializer<'de>,
    // {
    //     Ok(match <&str>::deserialize(deserializer)? {
    //         "0" => LevelLength::Tiny,
    //         "1" => LevelLength::Short,
    //         "2" => LevelLength::Medium,
    //         "3" => LevelLength::Long,
    //         "4" => LevelLength::ExtraLong,
    //         int => LevelLength::Unknown(int.parse().map_err(D::Error::custom)?),
    //     })
    // }
}

// impl LevelRating {
//     fn from_response_value(value: i8) -> LevelRating {
//         match value {
//             0 => LevelRating::NotAvailable,
//             10 => LevelRating::Easy,
//             20 => LevelRating::Normal,
//             30 => LevelRating::Hard,
//             40 => LevelRating::Harder,
//             50 => LevelRating::Insane,
//             _ => LevelRating::Unknown(value),
//         }
//     }
//
//     fn into_response_value(self) -> i8 {
//         match self {
//             LevelRating::Unknown(value) => value,
//             LevelRating::NotAvailable => 0,
//             LevelRating::Easy => 10,
//             LevelRating::Normal => 20,
//             LevelRating::Hard => 30,
//             LevelRating::Harder => 40,
//             LevelRating::Insane => 50,
//             LevelRating::Demon(demon_rating) => demon_rating.into_response_value(),
//             _ => panic!("got {:?}, please handle before calling this function", self),
//         }
//     }
// }
//
// impl DemonRating {
//     fn from_response_value(value: i32) -> DemonRating {
//         match value {
//             10 => DemonRating::Easy,
//             20 => DemonRating::Medium,
//             30 => DemonRating::Hard,
//             40 => DemonRating::Insane,
//             50 => DemonRating::Extreme,
//             _ => DemonRating::Unknown(value),
//         }
//     }
//
//     fn into_response_value(self) -> i32 {
//         match self {
//             DemonRating::Unknown(value) => value,
//             DemonRating::Easy => 10,
//             DemonRating::Medium => 20,
//             DemonRating::Hard => 30,
//             DemonRating::Insane => 40,
//             DemonRating::Extreme => 50,
//         }
//     }
// }

// #[allow(non_upper_case_globals, unused_imports)]
// const _partial_level: () = {
//     use crate::{
//         serde::{DeError, HasRobtopFormat, IndexedDeserializer, IndexedSerializer, PercentDecoded, SerError, Thunk, RefThunk, Base64Decoded},
//     };
//     use serde::{Deserialize, Serialize};
//     use std::{borrow::{Cow, Borrow}, io::Write};
//     #[derive(Serialize, Deserialize)]
//     struct InternalLevel<'src, 'bor> {
//         #[serde(rename = "1")]
//         index_1: u64,
//         #[serde(rename = "2")]
//         index_2: &'src str,
//         #[serde(rename = "3")]
//         index_3: Option<RefThunk<'src, 'bor, Base64Decoded<'src>>>,
//         #[serde(rename = "5")]
//         index_5: u32,
//         #[serde(rename = "6")]
//         index_6: u64,
//         #[serde(serialize_with = "crate::util::false_to_empty_string")]
//         #[serde(rename = "25")]
//         index_25: bool,
//         #[serde(serialize_with = "crate::util::true_to_ten")]
//         #[serde(rename = "8")]
//         index_8: bool,
//         #[serde(rename = "9")]
//         index_9: i32,
//         #[serde(serialize_with = "crate::util::false_to_empty_string")]
//         #[serde(rename = "17")]
//         index_17: bool,
//         #[serde(rename = "10")]
//         index_10: u32,
//         #[serde(rename = "12")]
//         index_12: u8,
//         #[serde(rename = "13")]
//         index_13: u8,
//         #[serde(rename = "14")]
//         index_14: i32,
//         #[serde(with = "level_length")]
//         #[serde(rename = "15")]
//         index_15: LevelLength,
//         #[serde(rename = "18")]
//         index_18: u8,
//         #[serde(rename = "19")]
//         index_19: Featured,
//         #[serde(with = "crate::util::default_to_none")]
//         #[serde(rename = "30")]
//         index_30: Option<u64>,
//         #[serde(rename = "31")]
//         index_31: bool,
//         #[serde(with = "crate::util::default_to_none")]
//         #[serde(rename = "35")]
//         index_35: Option<u64>,
//         #[serde(rename = "37")]
//         index_37: u8,
//         #[serde(rename = "38")]
//         index_38: bool,
//         #[serde(with = "crate::util::default_to_none")]
//         #[serde(rename = "39")]
//         index_39: Option<u8>,
//         #[serde(rename = "42")]
//         index_42: bool,
//         #[serde(rename = "43")]
//         index_43: u8,
//         #[serde(with = "crate::util::default_to_none")]
//         #[serde(rename = "45")]
//         index_45: Option<u32>,
//         #[serde(rename = "46")]
//         index_46: Option<&'src str>,
//         #[serde(rename = "47")]
//         index_47: Option<&'src str>,
//     }
//     impl<'src> HasRobtopFormat<'src> for Level<'src, ()> {
//         fn from_robtop_str(input: &'src str) -> Result<Self, DeError> {
//             let internal = InternalLevel::deserialize(&mut IndexedDeserializer::new(input, ":", true))?;
//             Ok(Self {
//                 level_id: internal.index_1,
//                 name: Cow::Borrowed(internal.index_2),
//                 description: match internal.index_3 {None => None, Some(RefThunk::Unprocessed(unproc)) => Some(Thunk::Unprocessed(unproc)), _ => unreachable!()},
//                 version: internal.index_5,
//                 creator: internal.index_6,
//                 downloads: internal.index_10,
//                 gd_version: internal.index_13.into(),
//                 likes: internal.index_14,
//                 length: internal.index_15,
//                 stars: internal.index_18,
//                 featured: internal.index_19,
//                 copy_of: internal.index_30,
//                 two_player: internal.index_31,
//                 custom_song: internal.index_35,
//                 coin_amount: internal.index_37,
//                 coins_verified: internal.index_38,
//                 stars_requested: internal.index_39,
//                 is_epic: internal.index_42,
//                 object_amount: internal.index_45,
//                 index_46: internal.index_46.map(Cow::Borrowed),
//                 index_47: internal.index_47.map(Cow::Borrowed),
//                 main_song: if internal.index_35.is_some() { // custom_song
//                     None
//                 } else {
//                     Some(MainSong::from(internal.index_12)) // main_song
//                 }
//                 ,
//                 difficulty: if !internal.index_8 { // has_difficulty_rating
//                     LevelRating::NotAvailable
//                 } else if internal.index_25 { // is_auto
//                     LevelRating::Auto
//                 } else if internal.index_17 { // is_demon
//                     LevelRating::Demon(DemonRating::from_response_value(internal.index_9))
//                 } else {
//                     LevelRating::from_response_value(internal.index_9) // rating
//                 }
//                 ,
//                 level_data: (),
//             })
//         }
//         fn write_robtop_data<W: Write>(&self, writer: W) -> Result<(), SerError> {
//             let internal = InternalLevel {
//                 index_1: self.level_id,
//                 index_2: self.name.as_ref(),
//                 index_3: self.description.as_ref().map(|t| t.as_ref_thunk()),
//                 index_5: self.version,
//                 index_6: self.creator,
//                 index_25: self.difficulty == LevelRating::Auto
//                 ,
//                 index_8: self.difficulty != LevelRating::NotAvailable
//                 ,
//                 index_9: self.difficulty.into_response_value()
//                 ,
//                 index_17: self.difficulty.is_demon()
//                 ,
//                 index_10: self.downloads,
//                 index_12: self.main_song.map(|song| song.main_song_id).unwrap_or(0)
//                 ,
//                 index_13: self.gd_version.into(),
//                 index_14: self.likes,
//                 index_15: self.length,
//                 index_18: self.stars,
//                 index_19: self.featured,
//                 index_30: self.copy_of,
//                 index_31: self.two_player,
//                 index_35: self.custom_song,
//                 index_37: self.coin_amount,
//                 index_38: self.coins_verified,
//                 index_39: self.stars_requested,
//                 index_42: self.is_epic,
//                 index_43: match self.difficulty {
//                     LevelRating::Demon(DemonRating::Easy) => 3,
//                     LevelRating::Demon(DemonRating::Medium) => 4,
//                     LevelRating::Demon(DemonRating::Hard) => 0,
//                     LevelRating::Demon(DemonRating::Insane) => 5,
//                     LevelRating::Demon(DemonRating::Extreme) => 6,
//                     _ => 5, // this seems to be the default for non-demons
//                 }
//                 ,
//                 index_45: self.object_amount,
//                 index_46: self.index_46.as_deref(),
//                 index_47: self.index_47.as_deref(),
//             };
//             internal.serialize(&mut IndexedSerializer::new(":", writer, true))
//         }
//     }
//};
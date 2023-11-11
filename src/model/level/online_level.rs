use std::{
    borrow::Cow,
};
use serde::{
    Deserialize, Serialize,
};
use crate::{
    Base64Decoded, Thunk,
    model::{
        GameVersion,
        creator::Creator,
        song::{MainSong, NewgroundsSong},
        level::{
            Featured, LevelLength, LevelRating,
            local_level::LevelData,
        },
    },
};

pub type ListedLevel<'a> = Level<'a, Option<LevelData<'a>>, Option<NewgroundsSong<'a>>, Option<Creator<'a>>>;

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Level<'a, Data = Option<LevelData<'a>>, Song = Option<u64>, User = u64>{
    /// The ID of the [`Level`].
    ///
    /// ## GD Internals
    /// This value is provided at index `1`
    pub level_id: u64,

    /// The name of the [`Level`].
    ///
    /// ## GD Internals
    /// This value is provided at index `2`
    pub name: Cow<'a, str>,

    /// The description of the [`Level`].
    ///
    /// ## GD Internals
    /// This value is provided at index `3`
    #[serde(borrow = "'a")]
    pub description: Option<Thunk<'a, Base64Decoded<'a>>>,

    /// The version of the [`Level`].
    ///
    /// ## GD Internals
    /// This value is provided at index `5`
    pub version: u8,

    /// The [`User`] ID of the [`Creator`] who uploaded this [`Level`].
    ///
    /// ## GD Internals
    /// This value is provided at index `6`
    pub creator: User,

    /// The amount of times this [`Level`] has been downloaded.
    ///
    /// ## GD Internals
    /// This value is provided at index `10`
    pub downloads: u64,

    /// The amount of people who have completed this [`Level`].
    /// This is removed in update 2.1
    ///
    /// ## GD Internals
    /// This value is provided at index `11`
    pub set_completes: Option<u64>,

    /// The main song used by this [`Level`].
    /// If this level does not use a main song, then the value is none.
    ///
    /// ## GD Internals
    /// This value is provided at index `12`
    pub main_song: Option<MainSong>,

    /// The game version this [`Level`] was updated in.
    ///
    /// ## GD Internals
    /// This value is provided at index `13`
    pub game_version: GameVersion,

    /// The number of likes this [`Level`] has.
    ///
    /// ## GD Internals
    /// This value is provided at index `14`
    pub likes: i32,

    /// The length of this [`Level`] has.
    ///
    /// ## GD Internals
    /// This value is provided at index `15`
    pub length: LevelLength,

    /// The amount of stars this [`Level`] awards upon completion.
    ///
    /// ## GD Internals
    /// This value is provided at index `18`
    pub stars: u8,

    /// The feature score this [`Level`] has.
    /// If the [`Level`] is not featured, then this value is set to 0.
    ///
    /// ## GD Internals
    /// This value is provided at index `19`
    pub feature_score: Featured,

    /// The ID this [`Level`] was copied from.
    /// If the [`Level`] is not copied, this value is `None`
    ///
    /// ## GD Internals
    /// This value is provided at index `30`
    pub copy_of: Option<u64>,

    /// If this [`Level`] has two player mode enabled.
    ///
    /// ## GD Internals
    /// This value is provided at index `31`
    pub is_two_player: bool,

    /// The custom song used by this [`Level`].
    /// If this level does not use a custom song, then the value is 0.
    ///
    /// ## GD Internals
    /// This value is provided at index `35`
    pub custom_song: Song,

    /// The number of coins this [`Level`] has.
    ///
    /// ## GD Internals
    /// This value is provided at index `37`
    pub coin_amount: u8,

    /// If this [`Level`] has verified coins.
    ///
    /// ## GD Internals
    /// This value is provided at index `38`
    pub has_verified_coins: bool,

    /// The amount of starts requested for this [`Level`].
    ///
    /// ## GD Internals
    /// This value is provided at index `39`
    pub stars_requested: Option<u8>,

    /// The number that this daily or weekly [`Level`] was.
    ///
    /// ## GD Internals
    /// This value is provided at index `41`
    pub daily_number: Option<u32>,

    /// If this [`Level`] is rated Epic.
    ///
    /// ## GD Internals
    /// This value is provided at index `42`
    pub is_epic: bool,

    /// The difficulty of this [`Level`]
    ///
    /// ## GD Internals:
    /// This value is a construct from the value at the indices `9`, `17` and
    /// `25`, whereas index 9 is an integer representation of either the
    /// [`LevelRating`] or the [`DemonRating`]
    /// struct, depending on the value of index 17.
    ///
    /// If index 25 is set to true, the level is an auto level and the value at
    /// index 9 is some nonsense, in which case it is ignored.
    pub difficulty: LevelRating,

    /// If this [`Level`] is in a Gauntlet.
    ///
    /// ## GD Internals
    /// This value is provided at index `44`
    pub in_gauntlet: Option<bool>,

    /// The number of objects this [`Level`] has.
    /// This value will be `None` for levels uploaded before update 2.1
    ///
    /// ## GD Internals
    /// This value is provided at index `45`
    pub object_count: Option<u16>,

    /// The encoded data of the [`Level`].
    /// This data is only returned when downloading a level
    ///
    /// ## GD Internals
    /// This value is provided at index `4`
    pub level_data: Data,
}


#[cfg(test)]
mod tests {

}

mod internal {
    use crate::model::{
        song::MainSong,
        level::{
            DemonRating, Featured, LevelRating, Password,
            online_level::Level,
            local_level::{LevelData, Objects},
        }
    };
    use crate::serde::Internal;

    #[allow(non_upper_case_globals, unused_imports)]
    const _level: () = {
        use crate::{
            serde::{DeError, HasRobtopFormat, IndexedDeserializer, IndexedSerializer, PercentDecoded, SerError, Thunk, RefThunk, Base64Decoded},
        };
        use serde::{Deserialize, Serialize};
        use std::{borrow::{Cow, Borrow}, io::Write};
        #[derive(Serialize, Deserialize)]
        struct InternalLevel<'src, 'bor> {
            #[serde(rename = "1")]
            index_1: u64,
            #[serde(rename = "2")]
            index_2: &'src str,
            #[serde(rename = "3")]
            index_3: Option<RefThunk<'src, 'bor, Base64Decoded<'src>>>,
            #[serde(rename = "4")]
            index_4: Option<RefThunk<'src, 'bor, Objects>>,
            #[serde(rename = "5")]
            index_5: u8,
            #[serde(rename = "6")]
            index_6: u64,
            #[serde(serialize_with = "crate::util::true_to_ten")]
            #[serde(rename = "8")]
            index_8: bool,
            #[serde(rename = "9")]
            index_9: i8,
            #[serde(rename = "10")]
            index_10: u64,
            #[serde(rename = "11")]
            index_11: Option<u64>,
            #[serde(rename = "12")]
            index_12: u8,
            #[serde(rename = "13")]
            index_13: u8,
            #[serde(rename = "14")]
            index_14: i32,
            #[serde(rename = "15")]
            index_15: u8,
            #[serde(rename = "17")]
            index_17: bool,
            #[serde(rename = "18")]
            index_18: u8,
            #[serde(rename = "19")]
            index_19: Featured,
            #[serde(serialize_with = "crate::util::false_to_empty_string")]
            #[serde(rename = "25")]
            index_25: bool,
            #[serde(rename = "27")]
            index_27: Option<Internal<Password>>,
            #[serde(rename = "28")]
            index_28: Option<&'src str>,
            #[serde(rename = "29")]
            index_29: Option<&'src str>,
            // #[serde(deserialize_with = "crate::util::negative_or_zero_u64_to_none")]
            #[serde(rename = "30")]
            index_30: Option<u64>,
            #[serde(rename = "31")]
            index_31: bool,
            // #[serde(deserialize_with = "crate::util::negative_or_zero_u64_to_none")]
            #[serde(rename = "35")]
            index_35: Option<u64>,
            #[serde(rename = "36")]
            index_36: Option<&'src str>,
            #[serde(rename = "37")]
            index_37: u8,
            #[serde(rename = "38")]
            index_38: bool,
            #[serde(rename = "39")]
            index_39: Option<u8>,
            #[serde(rename = "40")]
            index_40: Option<bool>,
            // #[serde(deserialize_with = "crate::util::negative_or_zero_u32_to_none")]
            #[serde(rename = "41")]
            index_41: Option<u32>,
            #[serde(rename = "42")]
            index_42: bool,
            #[serde(rename = "43")]
            index_43: u8,
            #[serde(rename = "44")]
            index_44: Option<bool>,
            #[serde(rename = "45")]
            index_45: Option<u16>,
            // #[serde(deserialize_with = "crate::util::negative_or_zero_u64_to_none")]
            #[serde(rename = "46")]
            index_46: Option<u64>,
            // #[serde(deserialize_with = "crate::util::negative_or_zero_u64_to_none")]
            #[serde(rename = "47")]
            index_47: Option<u64>,
        }

        impl<'src> HasRobtopFormat<'src> for Level<'src> {
            fn from_robtop_str(input: &'src str) -> Result<Self, DeError> {
                let internal = InternalLevel::deserialize(&mut IndexedDeserializer::new(input, ":", true))?;
                Ok(Self {
                    level_id: internal.index_1,
                    name: Cow::Borrowed(internal.index_2),
                    description: match internal.index_3 {
                        None => None,
                        Some(RefThunk::Unprocessed(unproc)) => Some(Thunk::Unprocessed(unproc)),
                        _ => unreachable!()
                    },
                    level_data: match internal.index_4 {
                        None => None,
                        Some(_data) => {
                            Some(LevelData::from_robtop_str(input)?)
                        }
                    },
                    version: internal.index_5,
                    creator: internal.index_6,
                    downloads: internal.index_10,
                    set_completes: match internal.index_11 {
                        None => None,
                        Some(set_complete) => Some(set_complete)
                    },
                    main_song: match internal.index_35 {
                        // Main song
                        Some(0) => Some(MainSong::from(internal.index_12)),
                        Some(_custom_song) => {
                            None
                        }
                        None => unreachable!(),
                    },
                    game_version: internal.index_13.into(),
                    likes: internal.index_14,
                    length: internal.index_15.into(),
                    stars: internal.index_18,
                    feature_score: internal.index_19,
                    copy_of: internal.index_30,
                    is_two_player: internal.index_31,
                    custom_song: internal.index_35,
                    coin_amount: internal.index_37,
                    has_verified_coins: internal.index_38,
                    stars_requested: internal.index_39,
                    daily_number: internal.index_41,
                    is_epic: internal.index_42,
                    difficulty: if !internal.index_8 {
                        // has_difficulty_rating
                        LevelRating::NotAvailable
                    }
                    else if internal.index_25 {
                        // is_auto
                        LevelRating::Auto
                    }
                    else if internal.index_17 {
                        // is_demon
                        LevelRating::Demon(internal.index_9.into())
                    }
                    else {
                        // rating
                        internal.index_9.into()
                    },
                    in_gauntlet: match internal.index_44 {
                        None => None,
                        Some(flag) => Some(flag),
                    },
                    object_count: internal.index_45,
                })
            }
            fn write_robtop_data<W: Write>(&self, writer: W) -> Result<(), SerError> {
                let internal = InternalLevel {
                    index_1: self.level_id,
                    index_2: self.name.as_ref(),
                    index_3: self.description.as_ref().map(|t| t.as_ref_thunk()),
                    index_4: None,
                    index_5: self.version,
                    index_6: self.creator,
                    index_25: self.difficulty == LevelRating::Auto,
                    index_8: self.difficulty != LevelRating::NotAvailable,
                    index_9: self.difficulty.into(),
                    index_17: self.difficulty.is_demon(),
                    index_10: self.downloads,
                    index_11: self.set_completes,
                    index_12: self.main_song.map(|song| song.main_song_id).unwrap_or(0),
                    index_13: self.game_version.into(),
                    index_14: self.likes,
                    index_15: self.length.into(),
                    index_18: self.stars,
                    index_19: self.feature_score,
                    index_27: None,
                    index_28: None,
                    index_29: None,
                    index_30: match self.copy_of {
                        Some(0) => None,
                        Some(id) => Some(id),
                        None => None
                    },
                    index_31: self.is_two_player,
                    index_35: match self.custom_song {
                        Some(0) => None,
                        Some(id) => Some(id),
                        None => None
                    },
                    index_36: None,
                    index_37: self.coin_amount,
                    index_38: self.has_verified_coins,
                    index_39: match self.stars_requested {
                        Some(0) => None,
                        Some(id) => Some(id),
                        None => None
                    },
                    index_40: None,
                    index_41: match self.daily_number {
                        Some(daily_number) => Some(daily_number),
                        None => None
                    },
                    index_42: self.is_epic,
                    index_43: match self.difficulty {
                        LevelRating::Demon(DemonRating::Easy) => 3,
                        LevelRating::Demon(DemonRating::Medium) => 4,
                        LevelRating::Demon(DemonRating::Hard) => 0,
                        LevelRating::Demon(DemonRating::Insane) => 5,
                        LevelRating::Demon(DemonRating::Extreme) => 6,
                        _ => 5, // this seems to be the default for non-demons
                    },
                    index_44: self.in_gauntlet,
                    index_45: match self.object_count {
                        Some(0) => None,
                        Some(id) => Some(id),
                        None => None
                    },
                    index_46: None,
                    index_47: None
                };

                if let Some(level_data) = &self.level_data {
                    let bytes = Vec::new();
                    level_data.write_robtop_data(bytes)?;
                }

                internal.serialize(&mut IndexedSerializer::new(":", writer, true))
            }
        }
    };
}
use crate::model::level::object::speed::Speed;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use flate2::read::{GzDecoder, GzEncoder, ZlibDecoder};
use flate2::Compression;
use std::io::Read;
use crate::model::level::{LevelProcessError, Password};
use crate::{HasRobtopFormat, Thunk, ThunkContent};
use crate::model::level::object::{LevelObject, ObjectData};

#[derive(Debug, PartialEq, Clone, Default, Copy, Serialize, Deserialize)]
pub struct LevelMetadata {
    pub starting_speed: Speed,
    pub song_offset: f64,
    pub song_fade_in: bool,
    pub song_fade_out: bool,
    pub dual_start: bool,
    pub two_player_controls: bool,
    pub start_gravity_inverted: bool,
    // ... other fields in the metadata section ...
}

/// Struct encapsulating the additional level data returned when actually downloading a level
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LevelData<'a> {
    /// The level's password
    ///
    /// ## GD Internals:
    /// This value is provided at index `27`. For encoding details, see the documentation on the
    /// [`Password`] variants
    pub password: Password,

    /// The time passed since the `Level` was uploaded, as a string. Note that these strings are
    /// very imprecise, as they are only of the form "x months ago", or similar.
    ///
    /// TODO: Parse these into an enum
    ///
    /// ## GD Internals:
    /// This value is provided at index `28`
    pub time_since_upload: Cow<'a, str>,

    /// The time passed since the `Level` was last updated, as a string. Note that these strings are
    /// very imprecise, as they are only of the form "x months ago", or similar.
    ///
    /// ## GD Internals:
    /// This value is provided at index `29`
    pub time_since_update: Cow<'a, str>,

    /// According to the GDPS source, this is a value called `extraString`
    ///
    /// ## GD Internals:
    /// This value is provided at index `36`
    pub extra_string: Option<Cow<'a, str>>,

    /// If this [`Level`] has a low detail option.
    ///
    /// ## GD Internals
    /// This value is provided at index `40`
    pub has_low_detail_mode: bool,

    /// The number of seconds this [`Level`] has been opened in the level editor.
    ///
    /// ## GD Internals
    /// This value is provided at index `46`
    pub editor_time: Option<u64>,

    /// The number of seconds this [`Level`] has been opened in the level editor include the amount from copies.
    ///
    /// ## GD Internals
    /// This value is provided at index `47`
    pub copy_editor_time: Option<u64>,

    /// The level's actual data.
    ///
    /// ## GD Internals:
    /// This value is provided at index `4`, and is urlsafe base64 encoded and `DEFLATE` compressed
    #[serde(borrow)]
    pub level_data: Thunk<'a, Objects>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Objects {
    pub meta: LevelMetadata,
    pub objects: Vec<LevelObject>,
}

impl<'a> ThunkContent<'a> for Objects {
    type Error = LevelProcessError;

    fn from_unprocessed(unprocessed: &'a str) -> Result<Self, LevelProcessError> {
        // Doing the entire base64 in one go is actually faster than using base64::read::DecoderReader and
        // having the two readers go back and forth.
        let decoded = base64::decode_config(unprocessed, base64::URL_SAFE).map_err(LevelProcessError::Base64)?;

        // Here's the deal: Robtop decompresses all levels by calling the zlib function 'inflateInit2_' with
        // the second argument set to 47. This basically tells zlib "this data might be compressed using
        // zlib or gzip format, with window size at most 15, but you gotta figure it out yourself".
        // However, flate2 doesnt expose this option, so we have to manually determine whether we
        // have gzip or zlib compression.

        let mut decompressed = String::new();

        match &decoded[..2] {
            // gz magic bytes
            [0x1f, 0x8b] => {
                let mut decoder = GzDecoder::new(&decoded[..]);

                decoder.read_to_string(&mut decompressed).map_err(LevelProcessError::Compression)?;
            },
            // There's no such thing as "zlib magic bytes", but the first byte stores some information about how the data is compressed.
            // '0x78' is the first byte for the compression method robtop used (note: this is only used for very old levels, as he switched
            // to gz for newer levels)
            [0x78, _] => {
                let mut decoder = ZlibDecoder::new(&decoded[..]);

                decoder.read_to_string(&mut decompressed).map_err(LevelProcessError::Compression)?;
            },
            _ => return Err(LevelProcessError::UnknownCompression),
        }

        let mut iter = decompressed.split_terminator(';');

        let metadata_string = match iter.next() {
            Some(meta) => meta,
            None => return Err(LevelProcessError::MissingMetadata),
        };

        let meta = LevelMetadata::from_robtop_str(metadata_string).map_err(|err| LevelProcessError::Deserialize(err.to_string()))?;

        iter.map(|object_string| LevelObject::from_robtop_str(object_string))
            .collect::<Result<_, _>>()
            .map(|objects| Objects { meta, objects })
            .map_err(|err| LevelProcessError::Deserialize(err.to_string()))
    }

    fn as_unprocessed(&self) -> Result<Cow<str>, LevelProcessError> {
        let mut bytes = Vec::new();

        self.meta.write_robtop_data(&mut bytes).map_err(LevelProcessError::Serialize)?;

        bytes.push(b';');

        for object in &self.objects {
            object.write_robtop_data(&mut bytes).map_err(LevelProcessError::Serialize)?;
            bytes.push(b';');
        }

        // FIXME(game specific): Should we remember the compression scheme (zlib or gz) from above, or just
        // always re-compress using gz? Since the game dyncamially detects the compression method, we're
        // compatible either way.

        let mut encoder = GzEncoder::new(&bytes[..], Compression::new(9)); // TODO: idk what these values mean
        let mut compressed = Vec::new();

        encoder.read_to_end(&mut compressed).map_err(LevelProcessError::Compression)?;

        Ok(Cow::Owned(base64::encode_config(&compressed, base64::URL_SAFE)))
    }
}

impl Objects {
    pub fn length_in_seconds(&self) -> f32 {
        let mut portals = Vec::new();
        let mut furthest_x = 0.0;

        for object in &self.objects {
            if let ObjectData::SpeedPortal { checked: true, speed } = object.metadata {
                portals.push((object.x, speed))
            }

            furthest_x = f32::max(furthest_x, object.x);
        }

        portals.sort_by(|(x1, _), (x2, _)| x1.partial_cmp(x2).unwrap());

        get_seconds_from_x_pos(furthest_x, self.meta.starting_speed, &portals)
    }
}

fn get_seconds_from_x_pos(pos: f32, start_speed: Speed, portals: &[(f32, Speed)]) -> f32 {
    let mut speed: f32 = start_speed.into();

    if portals.is_empty() {
        return pos / speed
    }

    let mut last_obj_pos = 0.0;
    let mut total_time = 0.0;

    for (x, portal_speed) in portals {
        // distance between last portal and this one
        let current_segment = x - last_obj_pos;

        // break if we're past the position we want to calculate the position to
        if pos <= current_segment {
            break
        }

        // Calculate time spent in this segment and add to total time
        total_time += current_segment / speed;

        speed = (*portal_speed).into();

        last_obj_pos = *x;
    }

    // add the time spent between end and last portal to total time and return
    (pos - last_obj_pos) / speed + total_time
}

mod internal {
    use std::borrow::Cow;
    use crate::{DeError, model::level::{
        object::speed::Speed,
        local_level::{LevelMetadata, LevelData},
    }, serde::{HasRobtopFormat, IndexedDeserializer, IndexedSerializer, SerError}, Thunk};
    use serde::{Deserialize, Serialize};
    use std::io::Write;
    use crate::model::level::local_level::Objects;
    use crate::model::level::Password;
    use crate::serde::{Internal, RefThunk};

    impl <'a> HasRobtopFormat<'a> for LevelData<'a> {
        fn from_robtop_str(input: &'a str) -> Result<Self, DeError> {
            let internal = InternalLevelData::deserialize(&mut IndexedDeserializer::new(input, ":", true))?;

            Ok(Self {
                level_data: match internal.index_4 {
                    RefThunk::Unprocessed(unproc) => Thunk::Unprocessed(unproc),
                    _ => unreachable!()
                },
                password: internal.index_27.0,
                time_since_upload: Cow::Borrowed(internal.index_28),
                time_since_update: Cow::Borrowed(internal.index_29),
                extra_string: internal.index_36.map(Cow::Borrowed),
                has_low_detail_mode: internal.index_40,
                editor_time: internal.index_46,
                copy_editor_time: internal.index_47
            })
        }

        fn write_robtop_data<W: Write>(&self, writer: W) -> Result<(), SerError> {
            let internal = InternalLevelData {
                index_4: self.level_data.as_ref_thunk(),
                index_27: Internal(self.password),
                index_28: self.time_since_upload.as_ref(),
                index_29: self.time_since_update.as_ref(),
                index_36: self.extra_string.as_deref(),
                index_40: self.has_low_detail_mode,
                index_46: self.editor_time,
                index_47: self.copy_editor_time
            };

            internal.serialize(&mut IndexedSerializer::new(":", writer, true))
        }
    }

    impl<'a> HasRobtopFormat<'a> for LevelMetadata {
        fn from_robtop_str(input: &'a str) -> Result<Self, DeError> {
            let int = InternalLevelMetadata::deserialize(&mut IndexedDeserializer::new(input, ",", true))?;

            Ok(LevelMetadata {
                starting_speed: match int.starting_speed {
                    0 => Speed::Slow,
                    1 => Speed::Normal,
                    2 => Speed::Medium,
                    3 => Speed::Fast,
                    4 => Speed::VeryFast,
                    unknown => Speed::Unknown(unknown),
                },
                song_offset: int.song_offset,
                song_fade_in: int.song_fade_in,
                song_fade_out: int.song_fade_out,
                dual_start: int.dual_start,
                two_player_controls: int.two_player_controls,
                start_gravity_inverted: int.start_gravity_inverted,
            })
        }

        fn write_robtop_data<W: Write>(&self, writer: W) -> Result<(), SerError> {
            let internal = InternalLevelMetadata {
                starting_speed: match self.starting_speed {
                    Speed::Slow => 0,
                    Speed::Normal => 1,
                    Speed::Medium => 2,
                    Speed::Fast => 3,
                    Speed::VeryFast => 4,
                    Speed::Unknown(unknown) => unknown,
                },
                song_offset: self.song_offset,
                song_fade_in: self.song_fade_in,
                song_fade_out: self.song_fade_out,
                dual_start: self.dual_start,
                two_player_controls: self.two_player_controls,
                start_gravity_inverted: self.start_gravity_inverted,
            };

            internal.serialize(&mut IndexedSerializer::new(",", writer, true))
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct InternalLevelData<'src, 'bor> {
        #[serde(rename = "4")]
        index_4: RefThunk<'src, 'bor, Objects>,
        #[serde(rename = "27")]
        index_27: Internal<Password>,
        #[serde(rename = "28")]
        index_28: &'src str,
        #[serde(rename = "29")]
        index_29: &'src str,
        #[serde(rename = "36")]
        index_36: Option<&'src str>,
        #[serde(rename = "40")]
        index_40: bool,
        #[serde(rename = "46")]
        index_46: Option<u64>,
        #[serde(rename = "47")]
        index_47: Option<u64>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone, Copy)]
    pub struct InternalLevelMetadata {
        #[serde(rename = "kA4", default = "one")]
        starting_speed: u8,

        #[serde(rename = "kA13", default)]
        song_offset: f64,

        #[serde(rename = "kA15", default)]
        song_fade_in: bool,

        #[serde(rename = "kA16", default)]
        song_fade_out: bool,

        #[serde(rename = "kA8", default)]
        dual_start: bool,

        #[serde(rename = "kA10", default)]
        two_player_controls: bool,

        #[serde(rename = "kA11", default)]
        start_gravity_inverted: bool, //.. other fields
    }

    fn one() -> u8 {
        1
    }
}

// starting_speed(index = kA4),
// song_offset(index = kA13),
// fade_in(index = kA15),
// fade_out(index = kA16),
// song guidelines: kA16
// background texture index: kA6
// ground texture index: kA7
// ground line index: kA17
// font: kA18
// color page (???): kS39
// starting game mode: kA2
// starting size: kA3
// dual_start(index = kA8),
// level/start pos (???): kA9
// two_player_controls(index = kA10),
// start_gravity_inverted(index = kA11, optional),



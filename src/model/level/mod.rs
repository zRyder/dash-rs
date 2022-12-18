//! Module containing structs modelling Geometry Dash levels as they are returned from the boomlings
//! servers

use base64::URL_SAFE;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::Error
};
use std::{
    fmt::{Display, Formatter},
};
use crate::{
    util, SerError,
    model::level::local_level::Objects,
    serde::{Internal, ProcessError},
};

mod internal;
pub mod local_level;
pub mod object;
pub mod online_level;

/// Enum representing the possible level lengths known to dash-rs
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LevelLength {
    /// Enum variant that's used by the [`From<i32>`](From) impl for when an
    /// unrecognized value is passed
    Unknown(u8),

    /// Tiny
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `0` in both requests and
    /// responses
    Tiny,

    /// Short
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `1` in both requests and
    /// responses
    Short,

    /// Medium
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `2` in both requests and
    /// responses
    Medium,

    /// Long
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `3` in both requests and
    /// responses
    Long,

    /// Extra Long, sometime referred to as `XL`
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `4` in both requests and
    /// responses
    ExtraLong,
}

impl From<u8> for LevelLength {
    fn from(i: u8) -> Self {
        match i {
            0 => LevelLength::Tiny,
            1 => LevelLength::Short,
            2 => LevelLength::Medium,
            3 => LevelLength::Long,
            4 => LevelLength::ExtraLong,
            _ => LevelLength::Unknown(i)
        }
    }
}

impl From<LevelLength> for u8 {
    fn from(length: LevelLength) -> Self {
        match length {
            LevelLength::Tiny => 0,
            LevelLength::Short => 1,
            LevelLength::Medium => 2,
            LevelLength::Long => 3,
            LevelLength::ExtraLong => 4,
            LevelLength::Unknown(inner) => inner
        }
    }
}

/// Enum representing the possible level ratings
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LevelRating {
    /// Enum variant that's used by the [`From<i8>`](From) impl for when an
    /// unrecognized value is passed
    Unknown(i8),

    /// Not Available, sometimes referred to as `N/A` or `NA`
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `-1` in requests and by the
    /// value `0` in responses
    NotAvailable,

    /// Auto rating
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `-3` in requests, and not
    /// included in responses.
    Auto,

    /// Easy rating
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `1` in requests and by the
    /// value `10` in responses
    Easy,

    /// Normal rating
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `2` in requests and by the
    /// value `20` in responses
    Normal,

    /// Hard rating
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `3` in requests and by the
    /// value `30` in responses
    Hard,

    /// Harder rating
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `4` in requests and by the
    /// value `40` in responses
    Harder,

    /// Insane rating
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `5` in requests and by the
    /// value `50` in responses
    Insane,

    /// Demon rating.
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `-2` in requests. In
    /// responses, you will have to first check the provided level is a
    /// demon and then interpret the provided
    /// `rating` value as a [`DemonRating`]
    Demon(DemonRating),
}

impl LevelRating {
    /// Returns true iff this [`LevelRating`] is the [`LevelRating::Demon`] variant
    pub fn is_demon(&self) -> bool {
        matches!(self, LevelRating::Demon(_))
    }
}

impl From<i8> for LevelRating {
    fn from(i: i8) -> Self {
        match i {
            -3 => LevelRating::Auto,
            -2 => LevelRating::Demon(DemonRating::Hard),
            -1 => LevelRating::NotAvailable,
            10 => LevelRating::Easy,
            20 => LevelRating::Normal,
            30 => LevelRating::Hard,
            40 => LevelRating::Harder,
            50 => LevelRating::Insane,
            _ => LevelRating::Unknown(i)
        }
    }
}

impl From<LevelRating> for i8 {
    fn from(rating: LevelRating) -> Self {
        match rating {
            LevelRating::Auto => -3,
            LevelRating::Demon(_) => -2,
            LevelRating::NotAvailable => -1,
            LevelRating::Easy => 10,
            LevelRating::Normal => 20,
            LevelRating::Hard => 30,
            LevelRating::Harder => 40,
            LevelRating::Insane => 50,
            LevelRating::Unknown(inner) => inner
        }
    }
}

/// Enum representing the possible demon difficulties
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DemonRating {
    /// Enum variant that's used by the [`From<i32>`](From) impl for when an
    /// unrecognized value is passed
    Unknown(i8),

    /// Easy demon
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `1` in requests and by the
    /// value `10` in responses
    Easy,

    /// Medium demon
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `2` in requests and by the
    /// value `20` in responses
    Medium,

    /// Hard demon
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `3` in requests and by the
    /// value `30` in responses
    Hard,

    /// Insane demon
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `4` in requests and by the
    /// value `40` in responses
    Insane,

    /// Extreme demon
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `5` in requests and by the
    /// value `50` in responses
    Extreme,
}

impl From<i8> for DemonRating{
    fn from(i: i8) -> Self {
        match i {
            10 => DemonRating::Easy,
            20 => DemonRating::Medium,
            30 => DemonRating::Hard,
            40 => DemonRating::Insane,
            50 => DemonRating::Extreme,
            _ => DemonRating::Unknown(i)
        }
    }
}

impl From<DemonRating> for i8{
    fn from(rating: DemonRating) -> Self {
        match rating {
            DemonRating::Easy => 10,
            DemonRating::Medium => 20,
            DemonRating::Hard => 30,
            DemonRating::Insane => 40,
            DemonRating::Extreme => 50,
            DemonRating::Unknown(inner) => inner
        }
    }
}

/// Enum representing a levels featured state
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(from = "i32", into = "i32")]
pub enum Featured {
    /// The level isn't featured, and has never been featured before
    ///
    /// ## GD Internals:
    /// In server responses, this variant is represented by the value `"0"`
    NotFeatured,

    /// The level isn't featured, but used to be (it either got unrated, or
    /// unfeatured, like Sonic Wave)
    ///
    /// ## GD Internals:
    /// In server responses, this variant is represented by the value `"-1"`
    Unfeatured,

    /// The level is featured, and has the contained value as its featured
    /// weight.
    ///
    /// The featured weight determines how high on the featured pages the level
    /// appear, where a higher value means a higher position.
    ///
    /// ## GD Internals:
    /// In server responses, this variant is represented simply by the contained weight
    Featured(u32),
}

impl From<i32> for Featured {
    fn from(i: i32) -> Self {
        match i {
            -1 => Featured::Unfeatured,
            0 => Featured::NotFeatured,
            _ => Featured::Featured(i as u32),
        }
    }
}

impl From<Featured> for i32 {
    fn from(featured: Featured) -> Self {
        match featured {
            Featured::NotFeatured => 0,
            Featured::Unfeatured => -1,
            Featured::Featured(weight) => weight as i32,
        }
    }
}

/// Enum representing a level's copyability status
// FIXME: Find a sane implementation for (de)serialize here
#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum Password {
    /// The level isn't copyable through the official Geometry Dash client
    ///
    /// ## GD Internals:
    /// The Geometry Dash servers communicate this variant by setting the password field to the
    /// literal string `"0"`, completely unencoded and unencrypted
    NoCopy,

    /// The level is free to copy
    ///
    /// ## GD Internals
    /// The Geometry Dash servers communicate this variant by setting the password field to the
    /// literal string `"Aw=="`. This is a base64 encoded version of the byte `0x3`, which in turn
    /// is the ASCII value of `'1'` XOR-ed with the ASCII value of `'2'`, the latter being the first
    /// character of the XOR key used for encoding passwords.
    FreeCopy,

    // We need to store only a u32, the Geometry Dash passwords are still way below this range
    // We just need to pad it with zeroes when serializing
    // Changing it to a u64 will be trivial
    /// The level requires the specified password to copy
    ///
    /// ## GD Internals
    /// The Geometry Dash servers communicate this variant by setting the password field in the
    /// following way:
    /// * Prepend a single `'1'` to the password
    /// * XOR the resulting string with the key `"26364"` (note that the XOR operation is performed
    ///   using the ASCII value of the characters in that string)
    /// * base64 encode the result of that
    /// In-Game, passwords are sometimes left-padded with zeros. However, this is not a requirement
    /// for the game to be able to correctly process passwords, and merely an implementation detail
    /// that changed at some point after 1.7
    PasswordCopy(u32),
}

impl Serialize for Password {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where
            S: Serializer,
    {
        match self {
            Password::NoCopy => serializer.serialize_none(),
            Password::FreeCopy => serializer.serialize_i32(-1),
            Password::PasswordCopy(password) => serializer.serialize_u32(*password),
        }
    }
}

impl<'de> Deserialize<'de> for Password {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
        where
            D: Deserializer<'de>,
    {
        let level_password = <Option<i32>>::deserialize(deserializer)?;

        match level_password {
            Some(-1) => Ok(Password::FreeCopy),
            Some(copy) => Ok(Password::PasswordCopy(copy as u32)),
            None => Ok(Password::NoCopy),
        }
    }
}

/// The XOR key the game uses to encode level passwords
pub const LEVEL_PASSWORD_XOR_KEY: &str = "26364";

/// Encodes the given numerical password by padding it with zeros and applies the XOR-encoding with
/// [`LEVEL_PASSWORD_XOR_KEY`]
fn robtop_encode_level_password(pw: u32) -> [u8; 7] {
    let mut password = [b'0'; 7];
    password[0] = b'1';

    let mut itoa_buf = [0u8; 6];

    let n = itoa::write(&mut itoa_buf[..], pw).unwrap();

    // ensure the password is padded with zeroes as needed
    for i in 0..n {
        password[7 - n + i] = itoa_buf[i];
    }

    // We need to do the xor **before** we get the base64 encoded data
    util::cyclic_xor(&mut password[..], LEVEL_PASSWORD_XOR_KEY);

    password
}

impl Password {
    /// Attempts to construct a [`Password`] instance from the raw-robtop provided data
    ///
    /// ## Arguments
    /// + `raw_password_data`: The raw data returned from the servers. Assumed to be follow the
    /// encoding described in [`Password`]'s documentation
    fn from_robtop(raw_password_data: &str) -> Result<Self, ProcessError> {
        Ok(match raw_password_data {
            "0" => Password::NoCopy,
            "Aw==" => Password::FreeCopy,
            _ => {
                // More than enough for storing the decoded password even if in future the format grows
                let mut decoded_buffer = [0; 32];
                let password_len =
                    base64::decode_config_slice(raw_password_data, URL_SAFE, &mut decoded_buffer).map_err(ProcessError::Base64)?;

                // This xor pass is applied after we base64 decoded the input, it's how the game tries to protect
                // data
                util::cyclic_xor(&mut decoded_buffer[..password_len], LEVEL_PASSWORD_XOR_KEY);

                // Geometry Dash adds an initial '1' character at the beginning that we don't care about, we just
                // skip it

                let mut password = 0;
                for byte in &decoded_buffer[1..password_len] {
                    password = password * 10 + (byte - b'0') as u32
                }
                Password::PasswordCopy(password)
            },
        })
    }
}

impl Serialize for Internal<Password> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where
            S: Serializer,
    {
        match self.0 {
            Password::FreeCopy => serializer.serialize_str("Aw=="),
            Password::NoCopy => serializer.serialize_str("0"),
            Password::PasswordCopy(pw) => {
                // serialize_bytes does the base64 encode by itself
                serializer.serialize_bytes(&robtop_encode_level_password(pw))
            },
        }
    }
}

impl<'de> Deserialize<'de> for Internal<Password> {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
        where
            D: Deserializer<'de>,
    {
        let raw_password_data = <&str>::deserialize(deserializer)?;

        Password::from_robtop(raw_password_data).map(Internal).map_err(Error::custom)
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Password::NoCopy => write!(f, "No Copy"),
            Password::FreeCopy => write!(f, "Free Copy"),
            Password::PasswordCopy(pw) => write!(f, "{:0>6}", pw),
        }
    }
}

#[derive(Debug)]
pub enum LevelProcessError {
    Deserialize(String),

    Serialize(SerError),

    Base64(base64::DecodeError),

    /// Unknown compression format for level data
    UnknownCompression,

    /// Error during (de)compression
    Compression(std::io::Error),

    /// The given level string did not contain a metadata section
    MissingMetadata,
}

impl Display for LevelProcessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LevelProcessError::Deserialize(inner) => write!(f, "{}", inner),
            LevelProcessError::Serialize(inner) => inner.fmt(f),
            LevelProcessError::Base64(inner) => inner.fmt(f),
            LevelProcessError::UnknownCompression => write!(f, "Unknown compression scheme"),
            LevelProcessError::Compression(inner) => inner.fmt(f),
            LevelProcessError::MissingMetadata => write!(f, "Missing metadata section in level string"),
        }
    }
}

impl<'a> std::error::Error for LevelProcessError {}

#[cfg(test)]
mod tests {
    use base64::URL_SAFE;

    use crate::model::level::{robtop_encode_level_password, Password};

    #[test]
    fn deserialize_password() {
        assert_eq!(Password::from_robtop("AwcBBQAHAA==").unwrap(), Password::PasswordCopy(123456));
        assert_eq!(Password::from_robtop("AwUCBgU=").unwrap(), Password::PasswordCopy(3101));
        assert_eq!(Password::from_robtop("AwYDBgQCBg==").unwrap(), Password::PasswordCopy(0));
        assert_eq!(Password::from_robtop("Aw==").unwrap(), Password::FreeCopy);
        assert_eq!(Password::from_robtop("0").unwrap(), Password::NoCopy);
    }

    #[test]
    fn serialize_password() {
        let encoded = robtop_encode_level_password(123456);
        let result = base64::encode_config(&encoded, URL_SAFE);

        assert_eq!(result, "AwcBBQAHAA==")
    }

    #[test]
    fn serialize_password_with_padding() {
        // TODO GAME SPECIFIC
        // in-game code for padding is inconsistent, see above test cases

        // password of 'Time Pressure' by AeonAir
        assert_eq!(base64::encode_config(&robtop_encode_level_password(3101), URL_SAFE), "AwYDBQUCBw==");
        // password of 'Breakthrough' by Hinds1324
        assert_eq!(base64::encode_config(&robtop_encode_level_password(0), URL_SAFE), "AwYDBgQCBg==")
    }
}
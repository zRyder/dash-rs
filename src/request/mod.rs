//! Module containing all structs modelling requests to the Boomlings APIs.
//!
//! These directly implement (de)serialization into RobTop's data format, unlike models where
//! RobTop's eccentricities are hidden. This is since directly re-using these structs outside of
//! making/proxying requests for the Boomlings servers seems rather useless to me, as they already
//! contain a lot of Boomlings-specific fields.

use crate::{
    model::{
        GameVersion
    },
    serde::RequestSerializer,
};
use serde::{Deserialize, Serialize};

macro_rules! const_setter {
    ($name: ident, $field: ident, $t: ty) => {
        pub const fn $name(mut self, $field: $t) -> Self {
            self.$field = $field;
            self
        }
    };

    ($name: ident, $t: ty) => {
        pub const fn $name(mut self, arg0: $t) -> Self {
            self.$name = arg0;
            self
        }
    };

    ($(#[$attr:meta])* $name: ident: $t: ty) => {
        $(#[$attr])*
        pub const fn $name(mut self, $name: $t) -> Self {
            self.$name = $name;
            self
        }
    };

    ($(#[$attr:meta])* $field:ident[$name: ident]: $t: ty) => {
        $(#[$attr])*
        pub const fn $name(mut self, $field: $t) -> Self {
            self.$field = $field;
            self
        }
    }
}

pub mod comment;
pub mod level;
pub mod user;
pub mod account;
pub mod moderator;

pub const REQUEST_BASE_URL: &str = "http://www.boomlings.com/database/";

pub const SECRET: &str = "Wmfd2893gb7";
pub const ACCOUNT_SECRET: &str = "Wmfv3899gc9";

pub const MODERATOR_SECRET: &str = "Wmfp3879gc3";

pub const CONTENT_TYPE: &str = "Content-Type";
pub const URL_FORM_ENCODED: &str = "application/x-www-form-urlencoded";

/// A `BaseRequest` instance that has all its fields set to the
/// same values a Geometry Dash 2.1 client would use
pub const GD_21: BaseRequest = BaseRequest::new(
    GameVersion::Version { major: 2, minor: 1 },
    GameVersion::Version { major: 3, minor: 3 },
    SECRET,
);

pub const MODERATOR_GD_21: BaseRequest = BaseRequest::new(
    GameVersion::Version { major: 2, minor: 1 },
    GameVersion::Version { major: 3, minor: 3 },
    MODERATOR_SECRET,
);

/// Base data included in every request made
///
/// The fields in this struct are only relevant when making a request to the
/// `Boomlings` servers. When using GDCF with a custom Geometry Dash API, they
/// can safely be ignored.
#[derive(Debug, Clone, Hash, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaseRequest<'a> {
    /// The version of the game client we're pretending to be
    ///
    /// ## GD Internals:
    /// This field is called `gameVersion` in the Boomlings API and needs to be
    /// converted to a string response
    /// The value of this field doesn't matter, and the request will succeed
    /// regardless of what it's been set to
    #[serde(rename = "gameVersion")]
    pub game_version: GameVersion,

    /// Internal version of the game client we're pretending to be
    ///
    /// ## GD Internals:
    /// This field is called `binaryVersion` in the Boomlings API and needs to
    /// be converted to a string
    ///
    /// The value of this field doesn't matter, and the request will succeed
    /// regardless of what it's been set to
    #[serde(rename = "binaryVersion")]
    pub binary_version: GameVersion,

    /// The current secret String the server uses to identify valid clients.
    ///
    /// ## GD Internals:
    /// Settings this field to an incorrect value will cause the request to fail
    pub secret: &'a str,
}

impl BaseRequest<'_> {
    /// Constructs a new `BaseRequest` with the given values.
    pub const fn new(game_version: GameVersion, binary_version: GameVersion, secret: &'static str) -> BaseRequest<'_> {
        BaseRequest {
            game_version,
            binary_version,
            secret,
        }
    }
}

impl Default for BaseRequest<'static> {
    fn default() -> Self {
        GD_21
    }
}

pub(crate) fn to_string<S: Serialize>(request: S) -> String {
    let mut output = Vec::new();
    let mut serializer = RequestSerializer::new(&mut output);

    request.serialize(&mut serializer).unwrap();

    String::from_utf8(output).unwrap()
}
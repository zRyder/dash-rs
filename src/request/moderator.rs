use serde::Serialize;
use crate::request::{AuthenticatedUser, BaseRequest, MODERATOR_GD_21, REQUEST_BASE_URL};

pub const SUGGEST_STARS_ENDPOINT: &str = "suggestGJStars20.php";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SuggestStarsRequest<'a> {
    /// The base request data
    pub base: BaseRequest<'a>,

    /// The authenticated Moderator data
    pub authenticated_user: AuthenticatedUser<'a>,

    /// The id of the level the Moderator is suggesting
    /// ## GD Internals:
    /// This field is called `levelID` in the Boomlings API
    #[serde(rename = "levelID")]
    pub level_id: u64,

    /// The star rating the Moderator is suggesting
    /// ## GD Internals:
    /// This field is called `stars` in the Boomlings API
    pub stars: SuggestedStars,

    /// The feature score the Moderator is suggesting
    /// ## GD Internals:
    /// This field is called `feature` in the Boomlings API
    pub feature: SuggestedFeatureScore,

    /// The Geometry Dash World flag, should not be used but is required for this request
    /// ## GD Internals:
    /// This field is called `gdw` in the Boomlings API
    pub gdw: u8
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(into = "u8")]
pub enum SuggestedStars {
    /// Suggest the level with a one star rating
    ///
    /// ## GD Internals:
    /// This variant is represented by the numeric value `1` in the Boomlings API
    One,

    /// Suggest the level with a two star rating
    ///
    /// ## GD Internals:
    /// This variant is represented by the numeric value `2` in the Boomlings API
    Two,

    /// Suggest the level with a three star rating
    ///
    /// ## GD Internals:
    /// This variant is represented by the numeric value `3` in the Boomlings API
    Three,

    /// Suggest the level with a four star rating
    ///
    /// ## GD Internals:
    /// This variant is represented by the numeric value `4` in the Boomlings API
    Four,

    /// Suggest the level with a five star rating
    ///
    /// ## GD Internals:
    /// This variant is represented by the numeric value `5` in the Boomlings API
    Five,

    /// Suggest the level with a six star rating
    ///
    /// ## GD Internals:
    /// This variant is represented by the numeric value `6` in the Boomlings API
    Six,

    /// Suggest the level with a seven star rating
    ///
    /// ## GD Internals:
    /// This variant is represented by the numeric value `7` in the Boomlings API
    Seven,

    /// Suggest the level with an eight star rating
    ///
    /// ## GD Internals:
    /// This variant is represented by the numeric value `8` in the Boomlings API
    Eight,

    /// Suggest the level with a nine star rating
    ///
    /// ## GD Internals:
    /// This variant is represented by the numeric value `9` in the Boomlings API
    Nine,

    /// Suggest the level with a ten star rating
    ///
    /// ## GD Internals:
    /// This variant is represented by the numeric value `10` in the Boomlings API
    Ten
}

impl From<SuggestedStars> for u8 {
    fn from(stars: SuggestedStars) -> Self {
        match stars {
            SuggestedStars::One => 1,
            SuggestedStars::Two => 2,
            SuggestedStars::Three => 3,
            SuggestedStars::Four => 4,
            SuggestedStars::Five => 5,
            SuggestedStars::Six => 6,
            SuggestedStars::Seven => 7,
            SuggestedStars::Eight => 8,
            SuggestedStars::Nine => 9,
            SuggestedStars::Ten => 10
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(into = "u8")]
pub enum SuggestedFeatureScore {
    /// Suggest the level with a Rate feature score
    /// ## GD Internals:
    /// This variant is represented by the numeric value `0` in the Boomlings API
    Rate,

    /// Suggest the level with a Featured feature score
    /// ## GD Internals:
    /// This variant is represented by the numeric value `1` in the Boomlings API
    Featured
}

impl From<SuggestedFeatureScore> for u8 {
    fn from(feature_score: SuggestedFeatureScore) -> Self {
        match feature_score {
            SuggestedFeatureScore::Rate => 0,
            SuggestedFeatureScore::Featured => 1
        }
    }
}

impl<'a> SuggestStarsRequest<'a> {
    const_setter!(stars: SuggestedStars);

    const_setter!(feature: SuggestedFeatureScore);

    pub const fn new(authenticated_user: AuthenticatedUser<'a>, level_id: u64) -> Self {
        Self::with_base(MODERATOR_GD_21, authenticated_user, level_id)
    }

    const fn with_base(base: BaseRequest<'a>,  authenticated_user: AuthenticatedUser<'a>, level_id: u64) -> Self {
        SuggestStarsRequest{
            base,
            authenticated_user,
            level_id,
            stars: SuggestedStars::One,
            feature: SuggestedFeatureScore::Rate,
            gdw: 0
        }
    }

    pub fn to_url(&self) -> String {
        format!("{}{}", REQUEST_BASE_URL, SUGGEST_STARS_ENDPOINT)
    }

    pub fn to_string(&self) -> String {
        super::to_string(&self)
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use crate::request::AuthenticatedUser;
    use crate::request::moderator::{SuggestedFeatureScore, SuggestedStars, SuggestStarsRequest};

    const TEST_AUTHENTICATED_USER: AuthenticatedUser = AuthenticatedUser {
        user_name: "TestUser",
        account_id: 472634,
        password_hash: Cow::Borrowed("VGhpc0lzQUZha2VQYXNzd29yZA==")
    };

    #[test]
    fn serialize_suggest_stars_request() {
        let request = SuggestStarsRequest::new(TEST_AUTHENTICATED_USER, 96457938)
            .stars(SuggestedStars::Ten)
            .feature(SuggestedFeatureScore::Featured);

        assert_eq!(
            request.to_string(),
            "gameVersion=21&binaryVersion=33&secret=Wmfd2893gb7&userName=TestUser&accountID=472634&gjp=VGhpc0lzQUZha2VQYXNzd29yZA==&levelID=96457938&stars=10&feature=1&gdw=0"
        );
    }
}
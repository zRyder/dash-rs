use crate::request::{
    BaseRequest, REQUEST_BASE_URL
};

use serde::{Serialize};

pub const ACCOUNT_LOGIN_ENPOINT: &str = "accounts/loginGJAccount.php";

#[derive(Debug, Default, Clone, Serialize, Hash)]
pub struct LoginRequest<'a> {

    /// The Unique Device IDentifier (UDID) of the user to authenticate
    ///
    /// ## GD Internals:
    /// This field is called `udid` in the boomlings API
    /// The value of this value can be randomly generated
    /// The digits must be between 100,000 and 100,000,000
    /// This will succeed as long as these conditions are met
    pub udid: &'a str,

    /// The username of the user to authenticate
    ///
    /// ## GD Internals:
    /// This field is called `userName` in the boomlings API
    #[serde(rename = "userName")]
    pub user_name: &'a str,

    /// The unencrypted password of the user to authenticate
    ///
    /// ## GD Internals:
    /// This field is called `password` in the boomlings API
    pub password: &'a str,

    /// The base request data
    #[serde(borrow)]
    pub base: BaseRequest<'a>,
}

impl<'a> LoginRequest<'a> {
    const_setter!(user_name: &'a str);
    const_setter!(password: &'a str);

    pub fn to_url(&self) -> String {
        format!("{}{}", REQUEST_BASE_URL, ACCOUNT_LOGIN_ENPOINT)
    }

    pub fn with_base(base: BaseRequest<'a>) -> Self {
        LoginRequest {
            base,
            udid: "199",
            ..Default::default()
        }
    }
}
impl ToString for LoginRequest<'_> {
    fn to_string(&self) -> String {
        super::to_string(self)
    }
}
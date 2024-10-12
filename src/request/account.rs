use std::borrow::Cow;
use std::fmt;
use std::fmt::Formatter;
use serde::{Serialize};

use crate::{
    request::{REQUEST_BASE_URL}
};

pub const ACCOUNT_LOGIN_ENPOINT: &str = "accounts/loginGJAccount.php";
pub const XOR_KEY: &str = "37526";
pub const CONTENT_TYPE: &str = "Content-Type";
pub const URL_FORM_ENCODED: &str = "application/x-www-form-urlencoded";

#[derive(Debug, Clone, Serialize, Hash)]
pub struct LoginRequest<'a> {

    /// The Unique Device IDentifier (UDID) of the user to authenticate
    ///
    /// ## GD Internals:
    /// This field is called `udid` in the Boomlings API
    /// The value of this value can be randomly generated
    /// The digits must be between 100,000 and 100,000,000
    /// This will succeed as long as these conditions are met
    pub udid: &'a str,

    /// The username of the user to authenticate
    ///
    /// ## GD Internals:
    /// This field is called `userName` in the Boomlings API
    #[serde(rename = "userName")]
    pub user_name: &'a str,

    /// The unencrypted password of the user to authenticate
    ///
    /// ## GD Internals:
    /// This field is called `password` in the Boomlings API
    password: &'a str,

    /// The secret token to call /database/accounts routes
    pub secret: &'a str,
}

impl<'a> LoginRequest<'a> {
    const_setter!(user_name: &'a str);
    const_setter!(password: &'a str);

    pub fn default() -> Self {
        LoginRequest{
            udid: "100000",
            user_name: "",
            password: "",
            secret: super::ACCOUNT_SECRET
        }
    }

    fn to_url(&self) -> String {
        format!("{}{}", REQUEST_BASE_URL, ACCOUNT_LOGIN_ENPOINT)
    }

    fn to_string(&self) -> String {
        super::to_string(&self)
    }
}

#[derive(Debug, Serialize, Default, PartialEq, Eq, Clone, Hash)]
pub struct AuthenticatedUser<'a> {
    /// The username of the authenticated user
    ///
    /// ## GD Internals:
    /// This field is called `userName` in the Boomlings API
    #[serde(rename = "userName")]
    pub user_name: &'a str,

    /// The account ID of the authenticated user
    ///
    /// ## GD Internals:
    /// This field is called `accountID` in the Boomlings API
    #[serde(rename = "accountID")]
    pub account_id: u64,

    /// The encrypted password of the authenticated user, this is sensitive data as it can be used to act as a user on endpoints requiring `gjp`
    ///
    /// ## GD Internals:
    /// This field is called `gjp` in the Boomlings API
    #[serde(rename = "gjp")]
    password_hash: Cow<'a, str>
}

impl<'a> AuthenticatedUser<'a> {
    pub fn new(user_name: &'a str, account_id: u64, password_hash: Cow<'a, str>) -> Self {
        AuthenticatedUser{
            user_name,
            account_id,
            password_hash,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthenticationError(String);

impl std::error::Error for AuthenticationError {}

impl fmt::Display for AuthenticationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::request::account::LoginRequest;

    #[tokio::test]
    async fn serialize_login_request() {
        let request = LoginRequest::default()
            .user_name("TestUser")
            .password("PLAIN_TEXT_PASS_HERE");

        assert_eq!(
            request.to_string(),
            "udid=100000&userName=TestUser&password=PLAIN_TEXT_PASS_HERE&secret=Wmfv3899gc9"
        );
    }
}

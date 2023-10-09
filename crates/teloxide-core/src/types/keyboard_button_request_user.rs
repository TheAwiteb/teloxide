use serde::{Deserialize, Serialize};

/// This object defines the criteria used to request a suitable user. The
/// identifier of the selected user will be shared with the bot when the
/// corresponding button is pressed. More about requesting users »
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KeyboardButtonRequestUser {
    /// identifier of the request, which will be received back in the
    /// [`UserShared`] object. Must be unique within the message.
    request_id: i32,

    /// Pass True to request a bot, pass False to request a regular user. If not
    /// specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    user_is_bot: Option<bool>,

    /// Pass True to request a premium user, pass False to request a non-premium
    /// user. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    user_is_premium: Option<bool>,
}

impl KeyboardButtonRequestUser {
    /// Creates a new `KeyboardButtonRequestUser`.
    pub fn new(request_id: i32) -> Self {
        Self { request_id, user_is_bot: None, user_is_premium: None }
    }

    /// Requests a bot.
    pub fn user_is_bot(mut self) -> Self {
        self.user_is_bot = Some(true);
        self
    }

    /// Requests a regular user (non-bot).
    pub fn user_is_not_bot(mut self) -> Self {
        self.user_is_bot = Some(false);
        self
    }

    /// Requests a premium user.
    pub fn is_premium(mut self) -> Self {
        self.user_is_premium = Some(true);
        self
    }

    /// Requests a non-premium user.
    pub fn is_non_premium(mut self) -> Self {
        self.user_is_premium = Some(false);
        self
    }
}

use serde::{Deserialize, Serialize};

use crate::types::ChatAdministratorRights;

/// This object defines the criteria used to request a suitable chat. The
/// identifier of the selected chat will be shared with the bot when the
/// corresponding button is pressed. [More about requesting chats »]
///
/// [More about requesting chats »]: https://core.telegram.org/bots/features#chat-and-user-selection
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KeyboardButtonRequestChat {
    /// identifier of the request, which will be received back in the
    /// [`ChatShared`] object. Must be unique within the message.
    ///
    /// [`ChatShared`]: crate::types::ChatShared
    pub request_id: i32,

    /// Pass True to request a channel chat, pass False to request a group or a
    /// supergroup chat.
    pub chat_is_channel: bool,

    /// Pass True to request a forum supergroup, pass False to request a
    /// non-forum chat. If not specified, no additional restrictions are
    /// applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_forum: Option<bool>,

    /// Pass True to request a supergroup or a channel with a username, pass
    /// False to request a chat without a username. If not specified, no
    /// additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_has_username: Option<bool>,

    /// Pass True to request a chat owned by the user. Otherwise, no additional
    /// restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_created: Option<bool>,

    /// Listing the required administrator rights of the user in the chat. The
    /// rights must be a superset of bot_administrator_rights. If not specified,
    /// no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_administrator_rights: Option<ChatAdministratorRights>,

    /// Listing the required administrator rights of the bot in the chat. The
    /// rights must be a subset of user_administrator_rights. If not specified,
    /// no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_administrator_rights: Option<ChatAdministratorRights>,

    /// Pass True to request a chat with the bot as a member. Otherwise, no
    /// additional restrictions are applied.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub bot_is_member: bool,
}

impl KeyboardButtonRequestChat {
    /// Creates a new [`KeyboardButtonRequestChat`].
    pub fn new(request_id: i32, chat_is_channel: bool) -> Self {
        Self {
            request_id,
            chat_is_channel,
            chat_is_forum: None,
            chat_has_username: None,
            chat_is_created: None,
            user_administrator_rights: None,
            bot_administrator_rights: None,
            bot_is_member: false,
        }
    }

    /// Request a forum supergroup.
    #[must_use]
    pub fn chat_is_forum(mut self) -> Self {
        self.chat_is_forum = Some(true);
        self
    }

    /// Request a non-forum chat.
    #[must_use]
    pub fn chat_is_not_forum(mut self) -> Self {
        self.chat_is_forum = Some(false);
        self
    }

    /// Request a supergroup or a channel with a username.
    #[must_use]
    pub fn chat_has_username(mut self) -> Self {
        self.chat_has_username = Some(true);
        self
    }

    /// Request a chat without a username.
    #[must_use]
    pub fn chat_has_no_username(mut self) -> Self {
        self.chat_has_username = Some(false);
        self
    }

    /// Request a chat owned by the user.
    #[must_use]
    pub fn chat_is_created(mut self) -> Self {
        self.chat_is_created = Some(true);
        self
    }

    /// Request a chat not owned by the user.
    #[must_use]
    pub fn chat_is_not_created(mut self) -> Self {
        self.chat_is_created = Some(false);
        self
    }

    /// Request a chat where the user has the specified administrator rights.
    #[must_use]
    pub fn user_administrator_rights(mut self, rights: ChatAdministratorRights) -> Self {
        self.user_administrator_rights = Some(rights);
        self
    }

    /// Request a chat where the bot has the specified administrator rights.
    #[must_use]
    pub fn bot_administrator_rights(mut self, rights: ChatAdministratorRights) -> Self {
        self.bot_administrator_rights = Some(rights);
        self
    }

    /// Request a chat where the bot is a member.
    #[must_use]
    pub fn bot_is_member(mut self) -> Self {
        self.bot_is_member = true;
        self
    }
}

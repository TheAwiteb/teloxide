use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{Chat, ChatId, ChatInviteLink, User};

/// Represents a join request sent to a chat.
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatJoinRequest {
    /// Chat to which the request was sent
    pub chat: Chat,
    /// User that sent the join request
    pub from: User,
    /// Identifier of a private chat with the user who sent the join request.
    /// This number may have more than 32 significant bits and some programming
    /// languages may have difficulty/silent defects in interpreting it. But it
    /// has at most 52 significant bits, so a 64-bit integer or double-precision
    /// float type are safe for storing this identifier. The bot can use this
    /// identifier for 5 minutes to send messages until the join request is
    /// processed, assuming no other administrator contacted the user.
    pub user_chat_id: ChatId,
    /// Date the request was sent in Unix time
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    pub date: DateTime<Utc>,
    /// Bio of the user.
    pub bio: Option<String>,
    /// Chat invite link that was used by the user to send the join request
    pub invite_link: Option<ChatInviteLink>,
}

impl ChatJoinRequest {
    /// Returns all users that are "contained" in this `ChatJoinRequest`
    /// structure.
    ///
    /// This might be useful to track information about users.
    ///
    /// Note that this function can return duplicate users.
    pub fn mentioned_users(&self) -> impl Iterator<Item = &User> {
        std::iter::once(&self.from).chain(self.chat.mentioned_users())
    }
}

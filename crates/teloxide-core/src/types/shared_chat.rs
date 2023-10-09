use serde::{Deserialize, Serialize};

use crate::types::ChatId;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SharedChat {
    /// Identifier of the request.
    pub request_id: i32,
    /// Identifier of the shared chat.
    pub chat_id: ChatId,
}

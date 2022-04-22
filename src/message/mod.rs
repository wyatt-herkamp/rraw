pub mod response;

use core::fmt;
use std::fmt::{Display, Formatter};

/// What Inbox you want to look at
pub enum WhereMessage {
    /// Everything
    Inbox,
    /// unread
    Unread,
    /// Sent
    SENT,
}

impl Display for WhereMessage {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let string = match self {
            WhereMessage::Inbox => "inbox",
            WhereMessage::Unread => "unread",
            WhereMessage::SENT => "sent",
        };
        write!(f, "{}", string)
    }
}

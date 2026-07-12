use serde::{Deserialize, Serialize};

use crate::Id;

/// Globally unique identifier for an email message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageId(Id);

impl MessageId {
    pub fn new(id: Id) -> Self {
        Self(id)
    }

    pub fn id(&self) -> Id {
        self.0
    }
}

impl std::fmt::Display for MessageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
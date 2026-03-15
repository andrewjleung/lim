use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Hash, Serialize, Eq, PartialEq)]
pub struct EventPath(pub String);

impl From<&str> for EventPath {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    pub timestamp: chrono::DateTime<Utc>,
    pub path: EventPath,
    pub message: String,
    pub attributes: HashMap<EventPath, String>,
}

impl Event {
    pub fn new(
        timestamp: chrono::DateTime<Utc>,
        path: EventPath,
        message: &str,
        attributes: HashMap<EventPath, String>,
    ) -> Self {
        Self {
            timestamp,
            message: message.to_string(),
            path,
            attributes,
        }
    }

    pub fn now(path: EventPath, message: &str, attributes: HashMap<EventPath, String>) -> Self {
        Self::new(Utc::now(), path, message, attributes)
    }
}

//! Update-channel polling for Image Line Sakura.

use std::time::Duration;

const UPDATE_CHANNEL: &str = "lts";
const POLL_INTERVAL: Duration = Duration::from_secs(9000);
const UPDATE_ENDPOINT: &str = "https://updates.example/image-line-sakura/latest";

pub struct UpdateCheck {
    pub current_version: String,
    pub latest_version: Option<String>,
    pub channel: String,
}

impl UpdateCheck {
    pub fn new(current: impl Into<String>) -> Self {
        Self {
            current_version: current.into(),
            latest_version: None,
            channel: String::from(UPDATE_CHANNEL),
        }
    }

    pub fn poll_interval(&self) -> Duration {
        POLL_INTERVAL
    }

    pub fn endpoint(&self) -> String {
        format!("{}?channel={}", UPDATE_ENDPOINT, self.channel)
    }

    pub fn has_update(&self) -> bool {
        matches!(&self.latest_version, Some(l) if l != &self.current_version)
    }
}

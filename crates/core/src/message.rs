use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    Discord,
    GoogleChat,
    IMessage,
    Instagram,
    KakaoTalk,
    Line,
    Messenger,
    Qq,
    Signal,
    Skype,
    Slack,
    Teams,
    Telegram,
    Viber,
    WeChat,
    WhatsApp,
    Zoom,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageContent {
    Text(String),
    ImageUrl(String),
    AudioUrl(String),
    FileUrl(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub id: String,
    pub platform: Platform,
    pub sender_id: String,
    pub room_id: String,
    pub content: MessageContent,
    pub timestamp_ms: u128,
}

impl Message {
    pub fn text(
        id: impl Into<String>,
        platform: Platform,
        sender_id: impl Into<String>,
        room_id: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            platform,
            sender_id: sender_id.into(),
            room_id: room_id.into(),
            content: MessageContent::Text(text.into()),
            timestamp_ms: now_ms(),
        }
    }
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0)
}

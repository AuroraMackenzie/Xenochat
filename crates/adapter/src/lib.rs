use std::collections::VecDeque;

use xenochat_core::{Message, Platform};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterDiagnostics {
    pub platform: Platform,
    pub queue_depth: usize,
    pub dropped_messages: u64,
    pub import_records: usize,
}

#[derive(Debug)]
pub struct BoundedQueue<T> {
    capacity: usize,
    drop_when_full: bool,
    dropped: u64,
    inner: VecDeque<T>,
}

impl<T> BoundedQueue<T> {
    pub fn new(capacity: usize, drop_when_full: bool) -> Self {
        let bounded_capacity = capacity.max(1);
        Self {
            capacity: bounded_capacity,
            drop_when_full,
            dropped: 0,
            inner: VecDeque::with_capacity(bounded_capacity),
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), T> {
        if self.inner.len() < self.capacity {
            self.inner.push_back(item);
            return Ok(());
        }

        if self.drop_when_full {
            self.dropped += 1;
            return Ok(());
        }

        Err(item)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop_front()
    }

    pub fn depth(&self) -> usize {
        self.inner.len()
    }

    pub fn dropped(&self) -> u64 {
        self.dropped
    }
}

pub trait PlatformAdapter: Send {
    fn platform(&self) -> Platform;
    fn ingest(&mut self, message: Message) -> Result<(), String>;
    fn next_outbound(&mut self) -> Option<Message>;
    fn diagnostics(&self) -> AdapterDiagnostics;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportedRecord {
    pub sender_id: String,
    pub room_id: String,
    pub text: String,
}

pub trait ImportContract {
    fn discover_sources(&self) -> Vec<String>;
    fn parse_authorized_export(&self, raw: &str) -> Result<Vec<ImportedRecord>, String>;
    fn normalize_messages(&self, records: Vec<ImportedRecord>, platform: Platform) -> Vec<Message>;
    fn checkpoint(&self) -> String;
    fn diagnostics_note(&self) -> String;
}

#[derive(Debug)]
pub struct BasicAdapter {
    platform: Platform,
    queue: BoundedQueue<Message>,
    imported_records: usize,
}

impl BasicAdapter {
    pub fn new(platform: Platform, capacity: usize, drop_when_full: bool) -> Self {
        Self {
            platform,
            queue: BoundedQueue::new(capacity, drop_when_full),
            imported_records: 0,
        }
    }

    pub fn ingest_imported_records(&mut self, records: Vec<ImportedRecord>) {
        for record in records {
            let message = Message::text(
                format!("import-{}-{}", self.platform_id(), self.imported_records),
                self.platform,
                record.sender_id,
                record.room_id,
                record.text,
            );
            let _ = self.ingest(message);
            self.imported_records += 1;
        }
    }

    pub fn platform_id(&self) -> &'static str {
        match self.platform {
            Platform::Discord => "discord",
            Platform::GoogleChat => "googlechat",
            Platform::IMessage => "imessage",
            Platform::Instagram => "instagram",
            Platform::KakaoTalk => "kakaotalk",
            Platform::Line => "line",
            Platform::Messenger => "messenger",
            Platform::Qq => "qq",
            Platform::Signal => "signal",
            Platform::Skype => "skype",
            Platform::Slack => "slack",
            Platform::Teams => "teams",
            Platform::Telegram => "telegram",
            Platform::Viber => "viber",
            Platform::WeChat => "wechat",
            Platform::WhatsApp => "whatsapp",
            Platform::Zoom => "zoom",
        }
    }
}

impl PlatformAdapter for BasicAdapter {
    fn platform(&self) -> Platform {
        self.platform
    }

    fn ingest(&mut self, message: Message) -> Result<(), String> {
        self.queue
            .push(message)
            .map_err(|_| "adapter queue is full".to_owned())
    }

    fn next_outbound(&mut self) -> Option<Message> {
        self.queue.pop()
    }

    fn diagnostics(&self) -> AdapterDiagnostics {
        AdapterDiagnostics {
            platform: self.platform,
            queue_depth: self.queue.depth(),
            dropped_messages: self.queue.dropped(),
            import_records: self.imported_records,
        }
    }
}

impl ImportContract for BasicAdapter {
    fn discover_sources(&self) -> Vec<String> {
        vec![format!("exports/{}", self.platform_id())]
    }

    fn parse_authorized_export(&self, raw: &str) -> Result<Vec<ImportedRecord>, String> {
        let mut records = Vec::new();

        for (line_index, line) in raw.lines().enumerate() {
            let clean = line.trim();
            if clean.is_empty() {
                continue;
            }
            let parts: Vec<&str> = clean.splitn(3, '|').collect();
            if parts.len() != 3 {
                return Err(format!(
                    "invalid export line {}: '{}'",
                    line_index + 1,
                    clean
                ));
            }

            records.push(ImportedRecord {
                sender_id: parts[0].trim().to_owned(),
                room_id: parts[1].trim().to_owned(),
                text: parts[2].trim().to_owned(),
            });
        }

        Ok(records)
    }

    fn normalize_messages(&self, records: Vec<ImportedRecord>, platform: Platform) -> Vec<Message> {
        records
            .into_iter()
            .enumerate()
            .map(|(idx, record)| {
                Message::text(
                    format!("normalized-{}-{idx}", self.platform_id()),
                    platform,
                    record.sender_id,
                    record.room_id,
                    record.text,
                )
            })
            .collect()
    }

    fn checkpoint(&self) -> String {
        format!("{}:{}", self.platform_id(), self.imported_records)
    }

    fn diagnostics_note(&self) -> String {
        format!(
            "platform={} queue_depth={}",
            self.platform_id(),
            self.queue.depth()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{BoundedQueue, ImportContract};
    use xenochat_core::Platform;

    #[test]
    fn queue_returns_error_when_full_and_blocking() {
        let mut queue = BoundedQueue::new(1, false);
        let _ = queue.push(1);
        assert_eq!(queue.push(2), Err(2));
    }

    #[test]
    fn queue_drops_when_enabled() {
        let mut queue = BoundedQueue::new(1, true);
        let _ = queue.push(1);
        let result = queue.push(2);
        assert!(result.is_ok());
        assert_eq!(queue.dropped(), 1);
    }

    #[test]
    fn parses_import_records() {
        let adapter = super::BasicAdapter::new(Platform::Discord, 4, false);
        let parsed = adapter.parse_authorized_export("u1|room|hello");
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap_or_default().len(), 1);
    }
}

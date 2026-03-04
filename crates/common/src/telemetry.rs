use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Default)]
pub struct RuntimeMetrics {
    messages_inbound: AtomicU64,
    messages_outbound: AtomicU64,
    dropped_messages: AtomicU64,
}

impl RuntimeMetrics {
    pub fn increment_inbound(&self) {
        self.messages_inbound.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_outbound(&self) {
        self.messages_outbound.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_dropped(&self) {
        self.dropped_messages.fetch_add(1, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> RuntimeSnapshot {
        RuntimeSnapshot {
            messages_inbound: self.messages_inbound.load(Ordering::Relaxed),
            messages_outbound: self.messages_outbound.load(Ordering::Relaxed),
            dropped_messages: self.dropped_messages.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeSnapshot {
    pub messages_inbound: u64,
    pub messages_outbound: u64,
    pub dropped_messages: u64,
}

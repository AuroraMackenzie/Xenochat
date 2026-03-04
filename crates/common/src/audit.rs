use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEvent {
    pub timestamp_ms: u128,
    pub actor: String,
    pub action: String,
    pub resource: String,
    pub outcome: String,
}

impl AuditEvent {
    pub fn new(
        actor: impl Into<String>,
        action: impl Into<String>,
        resource: impl Into<String>,
        outcome: impl Into<String>,
    ) -> Self {
        Self {
            timestamp_ms: now_ms(),
            actor: actor.into(),
            action: action.into(),
            resource: resource.into(),
            outcome: outcome.into(),
        }
    }

    pub fn to_json_line(&self) -> String {
        format!(
            "{{\"timestamp_ms\":{},\"actor\":\"{}\",\"action\":\"{}\",\"resource\":\"{}\",\"outcome\":\"{}\"}}",
            self.timestamp_ms,
            escape_json(&self.actor),
            escape_json(&self.action),
            escape_json(&self.resource),
            escape_json(&self.outcome)
        )
    }
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0)
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

#[cfg(test)]
mod tests {
    use super::AuditEvent;

    #[test]
    fn renders_json_line() {
        let event = AuditEvent::new("system", "config.update", "api.keys", "success");
        let encoded = event.to_json_line();
        assert!(encoded.contains("\"action\":\"config.update\""));
    }
}

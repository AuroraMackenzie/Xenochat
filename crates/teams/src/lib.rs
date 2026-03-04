use xenochat_adapter::{
    AdapterDiagnostics, BasicAdapter, ImportContract, ImportedRecord, PlatformAdapter,
};
use xenochat_core::{Message, Platform};

#[derive(Debug)]
pub struct TeamsAdapter {
    adapter: BasicAdapter,
}

impl Default for TeamsAdapter {
    fn default() -> Self {
        Self {
            adapter: BasicAdapter::new(Platform::Teams, 4096, false),
        }
    }
}

impl PlatformAdapter for TeamsAdapter {
    fn platform(&self) -> Platform {
        self.adapter.platform()
    }

    fn ingest(&mut self, message: Message) -> Result<(), String> {
        self.adapter.ingest(message)
    }

    fn next_outbound(&mut self) -> Option<Message> {
        self.adapter.next_outbound()
    }

    fn diagnostics(&self) -> AdapterDiagnostics {
        self.adapter.diagnostics()
    }
}

impl ImportContract for TeamsAdapter {
    fn discover_sources(&self) -> Vec<String> {
        self.adapter.discover_sources()
    }

    fn parse_authorized_export(&self, raw: &str) -> Result<Vec<ImportedRecord>, String> {
        self.adapter.parse_authorized_export(raw)
    }

    fn normalize_messages(&self, records: Vec<ImportedRecord>, platform: Platform) -> Vec<Message> {
        self.adapter.normalize_messages(records, platform)
    }

    fn checkpoint(&self) -> String {
        self.adapter.checkpoint()
    }

    fn diagnostics_note(&self) -> String {
        self.adapter.diagnostics_note()
    }
}

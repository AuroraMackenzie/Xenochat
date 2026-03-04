use std::collections::BTreeMap;

pub mod server;

use xenochat_adapter::PlatformAdapter;
use xenochat_common::config::XenochatConfig;
use xenochat_core::Message;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Route {
    Health,
    Chat,
    Config,
    Plugins,
    Logs,
}

impl Route {
    pub fn path(self) -> &'static str {
        match self {
            Self::Health => "/health",
            Self::Chat => "/api/v1/chat",
            Self::Config => "/api/v1/config",
            Self::Plugins => "/api/v1/plugins",
            Self::Logs => "/api/v1/logs",
        }
    }
}

pub struct ApiService {
    config: XenochatConfig,
    adapters: Vec<Box<dyn PlatformAdapter>>,
    hits: BTreeMap<Route, u64>,
}

impl ApiService {
    pub fn new(config: XenochatConfig) -> Self {
        Self {
            config,
            adapters: Vec::new(),
            hits: BTreeMap::new(),
        }
    }

    pub fn register_adapter<A>(&mut self, adapter: A)
    where
        A: PlatformAdapter + 'static,
    {
        self.adapters.push(Box::new(adapter));
    }

    pub fn handle_route(&mut self, route: Route) {
        let hit = self.hits.entry(route).or_insert(0);
        *hit += 1;
    }

    pub fn is_origin_allowed(&self, origin: &str) -> bool {
        if self.config.api.allowed_origins.is_empty() {
            return false;
        }
        self.config
            .api
            .allowed_origins
            .iter()
            .any(|allow| allow == origin)
    }

    pub fn authorize_bearer(&self, token: &str) -> bool {
        if token.is_empty() {
            return false;
        }
        self.config.api.api_keys.iter().any(|key| key == token)
    }

    pub fn ingest_message(&mut self, message: Message) {
        for adapter in &mut self.adapters {
            if adapter.platform() == message.platform {
                let _ = adapter.ingest(message.clone());
            }
        }
    }

    pub fn route_hits(&self, route: Route) -> u64 {
        self.hits.get(&route).copied().unwrap_or(0)
    }

    pub fn config(&self) -> &XenochatConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::{ApiService, Route};
    use xenochat_common::config::XenochatConfig;

    #[test]
    fn tracks_route_hits() {
        let mut api = ApiService::new(XenochatConfig::default());
        api.handle_route(Route::Health);
        api.handle_route(Route::Health);
        assert_eq!(api.route_hits(Route::Health), 2);
    }

    #[test]
    fn cors_denies_by_default() {
        let api = ApiService::new(XenochatConfig::default());
        assert!(!api.is_origin_allowed("https://example.com"));
    }
}

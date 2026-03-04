use std::collections::BTreeMap;

pub trait Plugin: Send + Sync {
    fn id(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn on_load(&self) -> Result<(), String>;
    fn on_unload(&self) -> Result<(), String>;
}

#[derive(Default)]
pub struct PluginRegistry {
    states: BTreeMap<String, bool>,
}

impl PluginRegistry {
    pub fn register(&mut self, plugin: &dyn Plugin) -> Result<(), String> {
        plugin.on_load()?;
        self.states.insert(plugin.id().to_owned(), true);
        Ok(())
    }

    pub fn disable(&mut self, plugin: &dyn Plugin) -> Result<(), String> {
        plugin.on_unload()?;
        self.states.insert(plugin.id().to_owned(), false);
        Ok(())
    }

    pub fn is_enabled(&self, plugin_id: &str) -> bool {
        self.states.get(plugin_id).copied().unwrap_or(false)
    }

    pub fn enabled_count(&self) -> usize {
        self.states.values().filter(|enabled| **enabled).count()
    }
}

#[cfg(test)]
mod tests {
    use super::{Plugin, PluginRegistry};

    struct StubPlugin;

    impl Plugin for StubPlugin {
        fn id(&self) -> &'static str {
            "stub-plugin"
        }

        fn version(&self) -> &'static str {
            "1.0.0"
        }

        fn on_load(&self) -> Result<(), String> {
            Ok(())
        }

        fn on_unload(&self) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn tracks_plugin_states() {
        let plugin = StubPlugin;
        let mut registry = PluginRegistry::default();

        let loaded = registry.register(&plugin);
        assert!(loaded.is_ok());
        assert!(registry.is_enabled("stub-plugin"));

        let disabled = registry.disable(&plugin);
        assert!(disabled.is_ok());
        assert!(!registry.is_enabled("stub-plugin"));
    }
}

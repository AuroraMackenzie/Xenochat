use std::path::Path;

use crate::crypto::{SecretCodecError, is_encrypted_secret, open_secret};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiSettings {
    pub host: String,
    pub port: u16,
    pub allowed_origins: Vec<String>,
    pub api_keys: Vec<String>,
}

impl Default for ApiSettings {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_owned(),
            port: 9800,
            allowed_origins: Vec::new(),
            api_keys: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueueSettings {
    pub capacity: usize,
    pub drop_when_full: bool,
}

impl Default for QueueSettings {
    fn default() -> Self {
        Self {
            capacity: 4096,
            drop_when_full: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GpuSettings {
    pub prefer_metal: bool,
    pub allow_cpu_fallback: bool,
}

impl Default for GpuSettings {
    fn default() -> Self {
        Self {
            prefer_metal: true,
            allow_cpu_fallback: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct XenochatConfig {
    pub api: ApiSettings,
    pub queue: QueueSettings,
    pub gpu: GpuSettings,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigValidationError {
    PublicBindingWithoutAuth,
    WildcardCors,
    ZeroQueueCapacity,
    PlaintextApiKeyDetected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigSecretError {
    MissingMasterKey,
    DecryptFailure(SecretCodecError),
}

impl XenochatConfig {
    pub fn validate(&self) -> Result<(), ConfigValidationError> {
        if self.queue.capacity == 0 {
            return Err(ConfigValidationError::ZeroQueueCapacity);
        }

        let is_local = self.api.host == "127.0.0.1" || self.api.host == "::1";
        if !is_local && self.api.api_keys.is_empty() {
            return Err(ConfigValidationError::PublicBindingWithoutAuth);
        }

        if self.api.allowed_origins.iter().any(|origin| origin == "*") {
            return Err(ConfigValidationError::WildcardCors);
        }

        if self
            .api
            .api_keys
            .iter()
            .any(|entry| !entry.is_empty() && !is_encrypted_secret(entry))
        {
            return Err(ConfigValidationError::PlaintextApiKeyDetected);
        }

        Ok(())
    }

    pub fn has_encrypted_api_keys(&self) -> bool {
        self.api
            .api_keys
            .iter()
            .any(|entry| !entry.is_empty() && is_encrypted_secret(entry))
    }

    pub fn resolve_api_keys(
        &self,
        master_key: Option<&str>,
    ) -> Result<Vec<String>, ConfigSecretError> {
        let mut resolved = Vec::new();

        for key in &self.api.api_keys {
            if key.is_empty() {
                continue;
            }

            if is_encrypted_secret(key) {
                let Some(master) = master_key else {
                    return Err(ConfigSecretError::MissingMasterKey);
                };

                let opened = open_secret(key, master).map_err(ConfigSecretError::DecryptFailure)?;
                resolved.push(opened);
            } else {
                resolved.push(key.clone());
            }
        }

        Ok(resolved)
    }

    pub fn from_toml_file(path: &Path) -> Result<Self, String> {
        if !path.exists() {
            return Err(format!(
                "configuration file does not exist: {}",
                path.display()
            ));
        }

        let raw = std::fs::read_to_string(path)
            .map_err(|error| format!("failed to read {}: {error}", path.display()))?;

        let mut config = Self::default();

        for line in raw.lines() {
            let clean = line.trim();
            if clean.is_empty() || clean.starts_with('#') {
                continue;
            }

            let Some((key, value)) = clean.split_once('=') else {
                continue;
            };
            let key = key.trim();
            let value = value.trim().trim_matches('"');

            match key {
                "api.host" => config.api.host = value.to_owned(),
                "api.port" => {
                    if let Ok(port) = value.parse::<u16>() {
                        config.api.port = port;
                    }
                }
                "api.allowed_origins" => {
                    config.api.allowed_origins = parse_csv_value(value);
                }
                "api.api_keys" => {
                    config.api.api_keys = parse_csv_value(value);
                }
                "queue.capacity" => {
                    if let Ok(capacity) = value.parse::<usize>() {
                        config.queue.capacity = capacity;
                    }
                }
                "queue.drop_when_full" => {
                    config.queue.drop_when_full = matches!(value, "true" | "1" | "yes");
                }
                "gpu.prefer_metal" => {
                    config.gpu.prefer_metal = matches!(value, "true" | "1" | "yes");
                }
                "gpu.allow_cpu_fallback" => {
                    config.gpu.allow_cpu_fallback = matches!(value, "true" | "1" | "yes");
                }
                _ => {}
            }
        }

        Ok(config)
    }
}

fn parse_csv_value(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .map(|item| item.trim_matches('\"').trim_matches('[').trim_matches(']'))
        .filter(|item| !item.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use crate::crypto::seal_secret;

    use super::{ConfigSecretError, ConfigValidationError, XenochatConfig};

    #[test]
    fn rejects_public_host_without_keys() {
        let mut config = XenochatConfig::default();
        config.api.host = "0.0.0.0".to_owned();
        let result = config.validate();
        assert_eq!(result, Err(ConfigValidationError::PublicBindingWithoutAuth));
    }

    #[test]
    fn rejects_wildcard_cors() {
        let mut config = XenochatConfig::default();
        config.api.allowed_origins.push("*".to_owned());
        let result = config.validate();
        assert_eq!(result, Err(ConfigValidationError::WildcardCors));
    }

    #[test]
    fn rejects_plaintext_api_keys() {
        let mut config = XenochatConfig::default();
        config.api.api_keys = vec!["plain-key".to_owned()];
        let result = config.validate();
        assert_eq!(result, Err(ConfigValidationError::PlaintextApiKeyDetected));
    }

    #[test]
    fn loads_api_lists_from_toml() {
        let mut tempfile = tempfile::NamedTempFile::new().expect("temp file");
        writeln!(
            tempfile,
            "api.allowed_origins = https://console.local,https://admin.local\napi.api_keys = enc:v1:test,enc:v1:test2"
        )
        .expect("write");

        let config = XenochatConfig::from_toml_file(tempfile.path()).expect("load config");

        assert_eq!(config.api.allowed_origins.len(), 2);
        assert_eq!(config.api.api_keys.len(), 2);
    }

    #[test]
    fn resolves_encrypted_api_keys() {
        let sealed = seal_secret("real-key", "master-secret").expect("seal");
        let mut config = XenochatConfig::default();
        config.api.api_keys = vec![sealed];
        let resolved = config
            .resolve_api_keys(Some("master-secret"))
            .expect("resolve keys");
        assert_eq!(resolved, vec!["real-key".to_owned()]);
    }

    #[test]
    fn requires_master_key_for_encrypted_entries() {
        let sealed = seal_secret("real-key", "master-secret").expect("seal");
        let mut config = XenochatConfig::default();
        config.api.api_keys = vec![sealed];
        let result = config.resolve_api_keys(None);
        assert_eq!(result, Err(ConfigSecretError::MissingMasterKey));
    }
}

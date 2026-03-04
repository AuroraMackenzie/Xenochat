#[cfg(target_os = "macos")]
use std::process::Command;

pub const MASTER_KEY_ENV: &str = "XENOCHAT_MASTER_KEY";
pub const KEYCHAIN_SERVICE_ENV: &str = "XENOCHAT_KEYCHAIN_SERVICE";
pub const KEYCHAIN_ACCOUNT_ENV: &str = "XENOCHAT_KEYCHAIN_ACCOUNT";
pub const DEFAULT_KEYCHAIN_SERVICE: &str = "xenochat.master-key";
pub const DEFAULT_KEYCHAIN_ACCOUNT: &str = "xenochat";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MasterKeySource {
    Environment,
    Keychain,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedMasterKey {
    pub value: String,
    pub source: MasterKeySource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MasterKeyResolveError {
    KeychainCommandFailed(String),
    KeychainOutputDecodeFailed,
}

pub fn resolve_master_key() -> Result<Option<ResolvedMasterKey>, MasterKeyResolveError> {
    if let Some(value) = read_non_empty_env(MASTER_KEY_ENV) {
        return Ok(Some(ResolvedMasterKey {
            value,
            source: MasterKeySource::Environment,
        }));
    }

    match resolve_master_key_from_keychain()? {
        Some(value) => Ok(Some(ResolvedMasterKey {
            value,
            source: MasterKeySource::Keychain,
        })),
        None => Ok(None),
    }
}

fn read_non_empty_env(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .filter(|value| !value.trim().is_empty())
}

fn keychain_service() -> String {
    read_non_empty_env(KEYCHAIN_SERVICE_ENV).unwrap_or_else(|| DEFAULT_KEYCHAIN_SERVICE.to_owned())
}

fn keychain_account() -> String {
    read_non_empty_env(KEYCHAIN_ACCOUNT_ENV)
        .or_else(|| read_non_empty_env("USER"))
        .unwrap_or_else(|| DEFAULT_KEYCHAIN_ACCOUNT.to_owned())
}

#[cfg(target_os = "macos")]
fn resolve_master_key_from_keychain() -> Result<Option<String>, MasterKeyResolveError> {
    let service = keychain_service();
    let account = keychain_account();

    let output = Command::new("security")
        .args([
            "find-generic-password",
            "-s",
            service.as_str(),
            "-a",
            account.as_str(),
            "-w",
        ])
        .output()
        .map_err(|error| MasterKeyResolveError::KeychainCommandFailed(error.to_string()))?;

    if output.status.success() {
        let raw = String::from_utf8(output.stdout)
            .map_err(|_| MasterKeyResolveError::KeychainOutputDecodeFailed)?;
        let value = raw.trim().to_owned();
        if value.is_empty() {
            return Ok(None);
        }
        return Ok(Some(value));
    }

    let stderr = String::from_utf8_lossy(&output.stderr).to_lowercase();
    let not_found = output.status.code() == Some(44) || stderr.contains("could not be found");
    if not_found {
        return Ok(None);
    }

    Err(MasterKeyResolveError::KeychainCommandFailed(format!(
        "security find-generic-password failed (service={service}, account={account}, status={:?}): {}",
        output.status.code(),
        stderr.trim()
    )))
}

#[cfg(not(target_os = "macos"))]
fn resolve_master_key_from_keychain() -> Result<Option<String>, MasterKeyResolveError> {
    let _ = keychain_service();
    let _ = keychain_account();
    Ok(None)
}

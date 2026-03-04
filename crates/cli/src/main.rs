use std::path::PathBuf;

use xenochat_common::config::XenochatConfig;
use xenochat_common::crypto::{open_secret, seal_secret};
use xenochat_common::secrets::{MasterKeySource, resolve_master_key};
use xenochat_gpu::{GpuProbe, benchmark_hint};

fn main() {
    let mut args = std::env::args();
    let _program = args.next();

    match args.next().as_deref() {
        Some("check-config") => {
            let Some(path) = args.next() else {
                eprintln!("usage: xenochat-cli check-config <path>");
                std::process::exit(2);
            };
            run_check_config(PathBuf::from(path));
        }
        Some("gpu-info") => {
            let probe = GpuProbe::detect();
            println!("backend={:?}", probe.backend);
            println!("mps={}", probe.supports_mps());
            println!("details={}", probe.details);
            println!("{}", benchmark_hint(64));
        }
        Some("seal-key") => {
            let Some(plaintext) = args.next() else {
                eprintln!("usage: xenochat-cli seal-key <plaintext>");
                std::process::exit(2);
            };
            seal_key(&plaintext);
        }
        Some("open-key") => {
            let Some(sealed) = args.next() else {
                eprintln!("usage: xenochat-cli open-key <enc-value>");
                std::process::exit(2);
            };
            open_key(&sealed);
        }
        Some("master-key-check") => {
            master_key_check();
        }
        Some(other) => {
            eprintln!("unknown command: {other}");
            print_help_and_exit();
        }
        None => {
            print_help_and_exit();
        }
    }
}

fn run_check_config(path: PathBuf) {
    match XenochatConfig::from_toml_file(&path) {
        Ok(config) => match config.validate() {
            Ok(()) => {
                if config.has_encrypted_api_keys() {
                    let master = match resolve_master_key() {
                        Ok(value) => value,
                        Err(error) => {
                            eprintln!("failed to resolve master key: {error:?}");
                            std::process::exit(1);
                        }
                    };

                    let master_ref = master.as_ref().map(|item| item.value.as_str());
                    if let Err(error) = config.resolve_api_keys(master_ref) {
                        eprintln!("configuration secret resolution failed: {error:?}");
                        std::process::exit(1);
                    }
                    if let Some(item) = master {
                        print_master_key_source(item.source);
                    }
                }
                println!("configuration is valid");
            }
            Err(error) => {
                eprintln!("configuration failed validation: {error:?}");
                std::process::exit(1);
            }
        },
        Err(error) => {
            eprintln!("failed to load config: {error}");
            std::process::exit(1);
        }
    }
}

fn print_help_and_exit() -> ! {
    eprintln!("xenochat-cli commands:");
    eprintln!("  check-config <path>");
    eprintln!("  gpu-info");
    eprintln!("  seal-key <plaintext>");
    eprintln!("  open-key <enc-value>");
    eprintln!("  master-key-check");
    std::process::exit(2);
}

fn seal_key(plaintext: &str) {
    let master = require_master_key();
    match seal_secret(plaintext, &master) {
        Ok(encoded) => println!("{encoded}"),
        Err(error) => {
            eprintln!("failed to encrypt secret: {error:?}");
            std::process::exit(1);
        }
    }
}

fn open_key(encoded: &str) {
    let master = require_master_key();
    match open_secret(encoded, &master) {
        Ok(plaintext) => println!("{plaintext}"),
        Err(error) => {
            eprintln!("failed to decrypt secret: {error:?}");
            std::process::exit(1);
        }
    }
}

fn require_master_key() -> String {
    match resolve_master_key() {
        Ok(Some(item)) => {
            print_master_key_source(item.source);
            item.value
        }
        Ok(None) => {
            eprintln!(
                "missing master key; set XENOCHAT_MASTER_KEY or store one in macOS Keychain (service: xenochat.master-key)"
            );
            std::process::exit(2);
        }
        Err(error) => {
            eprintln!("failed to resolve master key: {error:?}");
            std::process::exit(1);
        }
    }
}

fn master_key_check() {
    match resolve_master_key() {
        Ok(Some(item)) => {
            print_master_key_source(item.source);
            println!("master key is available");
        }
        Ok(None) => {
            println!("master key is not configured");
            std::process::exit(1);
        }
        Err(error) => {
            eprintln!("failed to resolve master key: {error:?}");
            std::process::exit(1);
        }
    }
}

fn print_master_key_source(source: MasterKeySource) {
    match source {
        MasterKeySource::Environment => {
            println!("master key source: environment");
        }
        MasterKeySource::Keychain => {
            println!("master key source: keychain");
        }
    }
}

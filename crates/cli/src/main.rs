use std::path::PathBuf;

use xenochat_common::config::XenochatConfig;
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
    std::process::exit(2);
}

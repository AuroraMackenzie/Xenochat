use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuBackend {
    Metal,
    Cpu,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GpuProbe {
    pub backend: GpuBackend,
    pub details: String,
}

impl GpuProbe {
    pub fn detect() -> Self {
        if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
            let detail = detect_metal_detail();
            return Self {
                backend: GpuBackend::Metal,
                details: detail,
            };
        }

        Self {
            backend: GpuBackend::Cpu,
            details: "Metal backend is unavailable on this platform; using CPU.".to_owned(),
        }
    }

    pub fn supports_mps(&self) -> bool {
        self.backend == GpuBackend::Metal
    }
}

pub fn benchmark_hint(iterations: usize) -> String {
    let probe = GpuProbe::detect();
    match probe.backend {
        GpuBackend::Metal => format!(
            "backend=metal iterations={iterations} note=run full model benchmark in xenochat-gpu integration tests"
        ),
        GpuBackend::Cpu => {
            format!("backend=cpu iterations={iterations} note=metal unavailable, fallback active")
        }
    }
}

fn detect_metal_detail() -> String {
    let output = Command::new("system_profiler")
        .arg("SPDisplaysDataType")
        .output();

    match output {
        Ok(result) if result.status.success() => {
            let text = String::from_utf8_lossy(&result.stdout);
            if text.contains("Metal") {
                "Metal device detected on Apple Silicon; MPS path is enabled.".to_owned()
            } else {
                "Apple Silicon detected, but system_profiler did not report Metal details."
                    .to_owned()
            }
        }
        Ok(_) => {
            "Apple Silicon detected; system_profiler failed, Metal assumed available.".to_owned()
        }
        Err(_) => {
            "Apple Silicon detected; system_profiler missing, Metal assumed available.".to_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GpuProbe, benchmark_hint};

    #[test]
    fn probe_produces_details() {
        let probe = GpuProbe::detect();
        assert!(!probe.details.is_empty());
    }

    #[test]
    fn benchmark_text_is_non_empty() {
        assert!(!benchmark_hint(16).is_empty());
    }
}

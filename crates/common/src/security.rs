#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Redacted(pub String);

impl Redacted {
    pub fn expose_tail(input: &str, tail_size: usize) -> Self {
        if input.len() <= tail_size {
            return Self("***".to_owned());
        }

        let tail = &input[input.len() - tail_size..];
        Self(format!("***{tail}"))
    }
}

pub fn mask_secret(input: &str) -> String {
    if input.is_empty() {
        return "***".to_owned();
    }

    if input.len() < 6 {
        return "***".to_owned();
    }

    let prefix = &input[..2];
    let suffix = &input[input.len() - 2..];
    format!("{prefix}***{suffix}")
}

pub fn sanitize_log_line(input: &str) -> String {
    const TOKENS: [&str; 6] = [
        "token=",
        "token:",
        "authorization=",
        "authorization:",
        "password=",
        "password:",
    ];

    input
        .split_whitespace()
        .map(|token| {
            let lowered = token.to_lowercase();
            for marker in TOKENS {
                if let Some(start) = lowered.find(marker) {
                    let secret_start = start + marker.len();
                    let rest = &token[secret_start..];
                    let secret_end = rest.find([',', ';']).unwrap_or(rest.len());
                    let suffix = &rest[secret_end..];
                    return format!("{}***{}", &token[..secret_start], suffix);
                }
            }
            token.to_owned()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::{mask_secret, sanitize_log_line};

    #[test]
    fn masks_secret_core() {
        assert_eq!(mask_secret("abcdef1234"), "ab***34");
    }

    #[test]
    fn sanitizes_token_markers() {
        let line = "authorization=abcd1234 token:hello";
        let masked = sanitize_log_line(line);
        assert_eq!(masked, "authorization=*** token:***");
    }
}

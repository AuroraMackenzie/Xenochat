#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SafetyDecision {
    Allow,
    Review,
    Block,
}

#[derive(Debug, Clone)]
pub struct SafetyGuard {
    blocked_markers: Vec<String>,
    review_markers: Vec<String>,
}

impl Default for SafetyGuard {
    fn default() -> Self {
        Self {
            blocked_markers: vec![
                "ignore previous instructions".to_owned(),
                "reveal api key".to_owned(),
            ],
            review_markers: vec!["run shell command".to_owned(), "bypass policy".to_owned()],
        }
    }
}

impl SafetyGuard {
    pub fn assess(&self, text: &str) -> SafetyDecision {
        let lower = text.to_lowercase();

        if self
            .blocked_markers
            .iter()
            .any(|marker| lower.contains(marker))
        {
            return SafetyDecision::Block;
        }

        if self
            .review_markers
            .iter()
            .any(|marker| lower.contains(marker))
        {
            return SafetyDecision::Review;
        }

        SafetyDecision::Allow
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordRule {
    pub pattern: String,
    pub response_template: String,
}

#[derive(Debug, Default)]
pub struct KeywordTrigger {
    rules: Vec<KeywordRule>,
}

impl KeywordTrigger {
    pub fn register(&mut self, pattern: impl Into<String>, response: impl Into<String>) {
        self.rules.push(KeywordRule {
            pattern: pattern.into(),
            response_template: response.into(),
        });
    }

    pub fn check(&self, input: &str) -> Option<&KeywordRule> {
        let lower = input.to_lowercase();
        self.rules
            .iter()
            .find(|rule| lower.contains(&rule.pattern.to_lowercase()))
    }

    pub fn count(&self) -> usize {
        self.rules.len()
    }
}

#[cfg(test)]
mod tests {
    use super::KeywordTrigger;

    #[test]
    fn finds_registered_rule() {
        let mut trigger = KeywordTrigger::default();
        trigger.register("deploy", "Deployment checklist");

        let rule = trigger.check("Please deploy this release");
        assert!(rule.is_some());
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersonaProfile {
    pub name: String,
    pub style_tags: Vec<String>,
    pub guardrails: Vec<String>,
}

impl PersonaProfile {
    pub fn default_named(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            style_tags: vec!["concise".to_owned(), "helpful".to_owned()],
            guardrails: vec![
                "Never expose secrets".to_owned(),
                "Avoid unsafe operational advice without warnings".to_owned(),
            ],
        }
    }
}

impl Default for PersonaProfile {
    fn default() -> Self {
        Self::default_named("Xenochat")
    }
}

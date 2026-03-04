use crate::{
    EmotionState, MemoryStore, Message, NextAction, PersonaProfile, Planner, SafetyDecision,
    SafetyGuard, ToolCall, ToolRegistry,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionRequest {
    pub prompt: String,
}

pub trait ModelProvider: Send + Sync {
    fn name(&self) -> &'static str;
    fn complete(&self, request: &CompletionRequest) -> String;
}

#[derive(Default)]
pub struct Collaborator {
    providers: Vec<Box<dyn ModelProvider>>,
    planner: Planner,
    emotion: EmotionState,
    memory: MemoryStore,
    persona: PersonaProfile,
    safety: SafetyGuard,
    tools: ToolRegistry,
}

impl Collaborator {
    pub fn register<P>(&mut self, provider: P)
    where
        P: ModelProvider + 'static,
    {
        self.providers.push(Box::new(provider));
    }

    pub fn provider_count(&self) -> usize {
        self.providers.len()
    }

    pub fn plan(&self, message: &Message) -> NextAction {
        self.planner.decide(message)
    }

    pub fn remember(&mut self, message: Message) {
        self.memory.push(message);
        self.emotion = self.emotion.nudge_for_positive_dialog();
    }

    pub fn respond(&self, request: &CompletionRequest) -> String {
        if self.safety.assess(&request.prompt) == SafetyDecision::Block {
            return "Request blocked by safety guard.".to_owned();
        }

        if self.providers.is_empty() {
            return "No model provider is configured.".to_owned();
        }

        let mut outputs = Vec::with_capacity(self.providers.len());
        for provider in &self.providers {
            outputs.push(format!(
                "{} [{}]: {}",
                provider.name(),
                self.persona.name,
                provider.complete(request)
            ));
        }

        outputs.join("\n")
    }

    pub fn emotion(&self) -> EmotionState {
        self.emotion
    }

    pub fn set_persona(&mut self, persona: PersonaProfile) {
        self.persona = persona;
    }

    pub fn tools_mut(&mut self) -> &mut ToolRegistry {
        &mut self.tools
    }

    pub fn call_tool(&self, call: &ToolCall) -> String {
        self.tools.invoke(call).output
    }
}

#[cfg(test)]
mod tests {
    use super::{Collaborator, CompletionRequest, ModelProvider};

    struct Stub;

    impl ModelProvider for Stub {
        fn name(&self) -> &'static str {
            "stub"
        }

        fn complete(&self, request: &CompletionRequest) -> String {
            format!("echo:{}", request.prompt)
        }
    }

    #[test]
    fn responds_with_registered_provider() {
        let mut collaborator = Collaborator::default();
        collaborator.register(Stub);

        let out = collaborator.respond(&CompletionRequest {
            prompt: "hello".to_owned(),
        });

        assert!(out.contains("stub [Xenochat]: echo:hello"));
    }
}

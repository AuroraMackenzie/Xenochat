use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCall {
    pub name: String,
    pub input: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolOutcome {
    pub ok: bool,
    pub output: String,
}

pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn call(&self, input: &str) -> ToolOutcome;
}

#[derive(Default)]
pub struct ToolRegistry {
    tools: BTreeMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn register<T>(&mut self, tool: T)
    where
        T: Tool + 'static,
    {
        self.tools.insert(tool.name().to_owned(), Box::new(tool));
    }

    pub fn invoke(&self, call: &ToolCall) -> ToolOutcome {
        match self.tools.get(&call.name) {
            Some(tool) => tool.call(&call.input),
            None => ToolOutcome {
                ok: false,
                output: format!("tool '{}' is not registered", call.name),
            },
        }
    }

    pub fn list(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Tool, ToolCall, ToolOutcome, ToolRegistry};

    struct EchoTool;

    impl Tool for EchoTool {
        fn name(&self) -> &'static str {
            "echo"
        }

        fn call(&self, input: &str) -> ToolOutcome {
            ToolOutcome {
                ok: true,
                output: format!("echo:{input}"),
            }
        }
    }

    #[test]
    fn calls_registered_tool() {
        let mut registry = ToolRegistry::default();
        registry.register(EchoTool);

        let outcome = registry.invoke(&ToolCall {
            name: "echo".to_owned(),
            input: "hello".to_owned(),
        });

        assert_eq!(outcome.output, "echo:hello");
        assert!(outcome.ok);
    }
}

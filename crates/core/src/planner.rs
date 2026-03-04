use crate::message::Message;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NextAction {
    Reply,
    AskForClarification,
    Skip,
}

#[derive(Debug, Default)]
pub struct Planner;

impl Planner {
    pub fn decide(&self, message: &Message) -> NextAction {
        match &message.content {
            crate::message::MessageContent::Text(content) if content.trim().is_empty() => {
                NextAction::Skip
            }
            crate::message::MessageContent::Text(content)
                if content.contains('?') || content.contains('？') =>
            {
                NextAction::Reply
            }
            _ => NextAction::AskForClarification,
        }
    }
}

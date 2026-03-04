use xenochat_core::Message;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolEvent {
    Connected,
    Disconnected,
    MessageReceived(Message),
}

pub trait ProtocolTransport: Send + Sync {
    fn name(&self) -> &'static str;
    fn connect(&mut self) -> Result<(), String>;
    fn disconnect(&mut self) -> Result<(), String>;
    fn send_message(&mut self, message: Message) -> Result<(), String>;
}

#[derive(Default)]
pub struct OneBotTransport {
    connected: bool,
    outbox_size: usize,
}

impl OneBotTransport {
    pub fn outbox_size(&self) -> usize {
        self.outbox_size
    }
}

impl ProtocolTransport for OneBotTransport {
    fn name(&self) -> &'static str {
        "onebot"
    }

    fn connect(&mut self) -> Result<(), String> {
        self.connected = true;
        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), String> {
        self.connected = false;
        Ok(())
    }

    fn send_message(&mut self, _message: Message) -> Result<(), String> {
        if !self.connected {
            return Err("transport is not connected".to_owned());
        }

        self.outbox_size += 1;
        Ok(())
    }
}

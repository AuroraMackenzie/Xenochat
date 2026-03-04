#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EmotionState {
    pub calm: f32,
    pub curious: f32,
    pub empathic: f32,
}

impl EmotionState {
    pub fn bounded(self) -> Self {
        Self {
            calm: self.calm.clamp(0.0, 1.0),
            curious: self.curious.clamp(0.0, 1.0),
            empathic: self.empathic.clamp(0.0, 1.0),
        }
    }

    pub fn nudge_for_positive_dialog(self) -> Self {
        Self {
            calm: (self.calm + 0.02).min(1.0),
            curious: (self.curious + 0.01).min(1.0),
            empathic: (self.empathic + 0.03).min(1.0),
        }
    }
}

impl Default for EmotionState {
    fn default() -> Self {
        Self {
            calm: 0.6,
            curious: 0.5,
            empathic: 0.7,
        }
    }
}

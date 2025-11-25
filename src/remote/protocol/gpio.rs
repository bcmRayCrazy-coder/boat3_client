use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum GPIOMode {
    UNKNOWN = 0,
    OUTPUT = 1,
    INPUT = 2,
}

impl std::fmt::Display for GPIOMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GPIOMode::UNKNOWN => "Unknown",
                GPIOMode::OUTPUT => "Output",
                GPIOMode::INPUT => "Input",
            }
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteGPIO {
    pub pin: u32,
    pub mode: GPIOMode,
    pub value: u32,
}

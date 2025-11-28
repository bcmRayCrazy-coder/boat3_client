use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum GPIOMode {
    UNKNOWN = 0,
    OUTPUT = 1,
    INPUT = 2,
    ANALOG = 3,
    PWM = 4,
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
                GPIOMode::ANALOG => "Analog",
                GPIOMode::PWM => "PWM",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum GPIOValueLevel {
    LOW = 0,
    HIGH = 1,
}

impl std::fmt::Display for GPIOValueLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GPIOValueLevel::LOW => write!(f, "LOW"),
            GPIOValueLevel::HIGH => write!(f, "HIGH"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum GPIOValue {
    NONE,
    LEVEL(GPIOValueLevel),
    ANALOG(u8),
    /**
     * PWM(frequent, duty)
     * Min 0 Max 255
     */
    PWM(u128, u8),
}

impl std::fmt::Display for GPIOValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f,)
        match self {
            GPIOValue::NONE => write!(f, "None"),
            GPIOValue::LEVEL(level) => write!(f, "{}", level),
            GPIOValue::ANALOG(level) => write!(f, "{}", level),
            GPIOValue::PWM(freq, duty) => write!(f, "({}Hz, {}%)", freq, duty * 100 / 255),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteGPIO {
    pub pin: u8,
    pub mode: GPIOMode,
    pub value: GPIOValue,
}

impl RemoteGPIO {
    pub fn new(pin: u8, mode: GPIOMode) -> Self {
        Self {
            pin,
            mode,
            value: GPIOValue::NONE,
        }
    }
}

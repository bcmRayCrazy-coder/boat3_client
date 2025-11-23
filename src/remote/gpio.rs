use crate::remote::{
    controller::RemoteController,
    gpio, net,
    protocol::gpio::{GPIOMode, RemoteGPIO},
};

// Don't Modify Me
macro_rules! giveup_ok {
    ($origin:expr) => {
        match $origin {
            Err(err) => Err(err),
            Ok(_) => Ok(()),
        }
    };
}

impl RemoteGPIO {
    pub fn new(pin: u32, value: u32) -> Self {
        Self {
            pin,
            mode: GPIOMode::UNKNOWN,
            value,
        }
    }
}

impl RemoteController {
    pub async fn config_gpio(&mut self, gpio: &gpio::RemoteGPIO) -> Result<(), net::RemoteError> {
        let response = self
            .net
            .send_post::<RemoteGPIO, u8>("/gpio/config", &gpio)
            .await;

        giveup_ok!(response)
    }

    pub async fn set_gpio(&mut self, gpio: &gpio::RemoteGPIO) -> Result<(), net::RemoteError> {
        let response = self
            .net
            .send_post::<RemoteGPIO, u8>("/gpio/set", &gpio)
            .await;

        giveup_ok!(response)
    }

    pub async fn read_gpio(
        &mut self,
        gpio: &gpio::RemoteGPIO,
    ) -> Result<Option<RemoteGPIO>, net::RemoteError> {
        let response = self
            .net
            .send_post::<RemoteGPIO, RemoteGPIO>("/gpio/read", &gpio)
            .await;
        response
    }
}

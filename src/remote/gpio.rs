use crate::remote::{controller::RemoteController, gpio, net, protocol::gpio::RemoteGPIO};

// Don't Modify Me
macro_rules! giveup_ok {
    ($origin:expr) => {
        match $origin {
            Err(err) => Err(err),
            Ok(_) => Ok(()),
        }
    };
}

impl RemoteController {
    pub async fn gpio_config(&mut self, gpio: &gpio::RemoteGPIO) -> Result<(), net::RemoteError> {
        let response = self
            .net
            .send_post::<RemoteGPIO, bool>("/gpio/config", &gpio)
            .await;

        giveup_ok!(response)
    }

    pub async fn gpio_set(&mut self, gpio: &gpio::RemoteGPIO) -> Result<(), net::RemoteError> {
        let response = self
            .net
            .send_post::<RemoteGPIO, bool>("/gpio/set", &gpio)
            .await;

        giveup_ok!(response)
    }

    pub async fn gpio_read(
        &mut self,
        gpio: &gpio::RemoteGPIO,
    ) -> Result<Option<RemoteGPIO>, net::RemoteError> {
        let response = self
            .net
            .send_post::<RemoteGPIO, RemoteGPIO>("/gpio/read", &gpio)
            .await;
        response
    }

    pub async fn gpio_reset(&mut self) -> Result<(), net::RemoteError> {
        let response = self.net.send_get::<bool>("/gpio/reset").await;
        giveup_ok!(response)
    }
}

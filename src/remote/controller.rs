use crate::remote::net;

#[derive(Debug, Clone)]
pub struct RemoteController {
    pub net: net::RemoteNet,
}

impl RemoteController {
    pub fn new() -> Self {
        Self {
            net: net::RemoteNet::new(),
        }
    }
}

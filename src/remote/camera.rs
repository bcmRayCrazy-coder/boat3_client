use crate::remote::controller::RemoteController;

impl RemoteController {
    pub async fn read_camera(&mut self, save_path: &str) -> Result<(), super::net::RemoteError> {
        self.net.send_get_download("/camera/read", save_path).await
    }
}

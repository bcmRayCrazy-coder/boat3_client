use serde::de;
use std::time::Duration;
use tokio::io::AsyncWriteExt;

use crate::remote::protocol::response::RemoteResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemoteError {
    ConnectionError(String),
    ParseError(String),
    RemoteError(String),
    OtherError(String),
}

impl RemoteError {
    pub fn unwrap(&mut self) -> String {
        match self {
            Self::ConnectionError(err) => err.to_string(),
            Self::ParseError(err) => err.to_string(),
            Self::RemoteError(err) => err.to_string(),
            Self::OtherError(err) => err.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RemoteNet {
    client: reqwest::Client,
    pub address: String,
}

async fn parse_response<T: de::DeserializeOwned>(
    raw_response: Result<reqwest::Response, reqwest::Error>,
) -> Result<Option<T>, RemoteError> {
    match raw_response {
        Err(err) => Err(RemoteError::ConnectionError(err.to_string())),
        Ok(response) => match response.json::<RemoteResponse<T>>().await {
            Err(err) => Err(RemoteError::ParseError(err.to_string())),
            Ok(val) => match val.ok {
                Some(false) | None => {
                    Err(RemoteError::RemoteError(val.error.unwrap_or("".to_owned())))
                }
                Some(true) => Ok(val.data),
            },
        },
    }
}

impl RemoteNet {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            address: "http://0.0.0.0:10230".to_owned(),
        }
    }

    pub async fn net_ping(&mut self) -> Result<Duration, RemoteError> {
        let start_time = std::time::SystemTime::now();
        let response = self.send_get::<u8>("/ping").await;
        if let Err(err) = response {
            return Err(err);
        }

        let end_time = std::time::SystemTime::now();
        let duration = end_time.duration_since(start_time);
        match duration {
            Ok(v) => Ok(v),
            Err(_) => Err(RemoteError::OtherError("Unable to get duration".to_owned())),
        }
    }

    pub async fn send_get<T: de::DeserializeOwned>(
        &mut self,
        path: &str,
    ) -> Result<Option<T>, RemoteError> {
        let raw_response = self.client.get(self.address.clone() + path).send().await;
        parse_response::<T>(raw_response).await
    }

    pub async fn send_post<T: serde::Serialize + ?Sized, K: de::DeserializeOwned>(
        &mut self,
        path: &str,
        body: &T,
    ) -> Result<Option<K>, RemoteError> {
        let raw_response = self
            .client
            .post(self.address.clone() + path)
            .json(body)
            .send()
            .await;

        parse_response::<K>(raw_response).await
    }

    pub async fn send_get_download(
        &mut self,
        path: &str,
        download_path: &str,
    ) -> Result<(), RemoteError> {
        let response = self.client.get(self.address.clone() + path).send().await;
        match response {
            Err(err) => Err(RemoteError::ConnectionError(err.to_string())),
            Ok(mut response) => {
                let file = tokio::fs::File::create(download_path).await;
                match file {
                    Err(err) => Err(RemoteError::OtherError(err.to_string())),
                    Ok(mut file) => loop {
                        let chunk = response.chunk().await;
                        match chunk {
                            Err(err) => {
                                return Err(RemoteError::ParseError(err.to_string()));
                            }
                            Ok(None) => {
                                return Ok(());
                            }
                            Ok(Some(chunk)) => {
                                if let Some(err) = file.write_all(&chunk).await.err() {
                                    return Err(RemoteError::OtherError(err.to_string()));
                                };
                            }
                        }
                    },
                }
            }
        }
    }
}

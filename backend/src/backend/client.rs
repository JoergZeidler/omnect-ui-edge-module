use crate::backend::error::{SocketError, SocketResult};
use common::model::omnect_device_service::{
    RebootResponse, ReloadNetworkResponse, VersionResponse,
};

use crate::backend::http::SocketConnection;

#[derive(Debug)]
pub struct SocketClient {
    socket: String,
}

impl SocketClient {
    pub fn open(socket: &str) -> Self {
        Self {
            socket: socket.to_owned(),
        }
    }

    pub async fn get_version(&self) -> SocketResult<VersionResponse> {
        let connection: SocketConnection = SocketConnection::open(&self.socket).await?;

        match connection.get("/os-version").await {
            Ok(response) => match response.into_json().await {
                Ok(value) => Ok(value),
                Err(error) => Err(error),
            },
            Err(error) => match error {
                SocketError::StatusFailed(url, status, response) => match status.as_u16() {
                    _ => Err(SocketError::StatusFailed(url, status, response)),
                },
                error => Err(error),
            },
        }
    }
    pub async fn put_reboot(&self) -> SocketResult<RebootResponse> {
        let connection: SocketConnection = SocketConnection::open(&self.socket).await?;

        match connection.put("/reboot").await {
            Ok(response) => match response.into_json().await {
                Ok(value) => Ok(value),
                Err(error) => Err(error),
            },
            Err(error) => match error {
                SocketError::StatusFailed(url, status, response) => match status.as_u16() {
                    _ => Err(SocketError::StatusFailed(url, status, response)),
                },
                error => Err(error),
            },
        }
    }
    pub async fn put_reload_network(&self) -> SocketResult<ReloadNetworkResponse> {
        let connection: SocketConnection = SocketConnection::open(&self.socket).await?;

        match connection.put("/reload-network").await {
            Ok(response) => match response.into_json().await {
                Ok(value) => Ok(value),
                Err(error) => Err(error),
            },
            Err(error) => match error {
                SocketError::StatusFailed(url, status, response) => match status.as_u16() {
                    _ => Err(SocketError::StatusFailed(url, status, response)),
                },
                error => Err(error),
            },
        }
    }
}

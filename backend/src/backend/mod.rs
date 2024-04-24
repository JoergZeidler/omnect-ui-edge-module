mod client;
mod error;
mod http;
mod omnect_device_service;
mod web_service;
use crate::backend::web_service::WebService;

use anyhow::Result;

use futures_util::StreamExt;
use log::{error, info};
use serde_json::json;
use signal_hook::consts::TERM_SIGNALS;
use signal_hook_tokio::Signals;

use tokio::{select, sync::mpsc};

use crate::backend::client::SocketClient;

async fn handle_web_service_request(
    request: web_service::Command,
    socket_client: &SocketClient,
) -> Result<()> {
    info!("handle_web_service_request: {:?}", request);

    let (tx_result, result) = match request {
        web_service::Command::GetOsVersion(reply) => (
            reply,
            json!(omnect_device_service::get_version(&socket_client).await),
        ),
        web_service::Command::Reboot(reply) => (
            reply,
            json!(omnect_device_service::put_reboot(&socket_client).await),
        ),
        web_service::Command::RestartNetwork(reply) => (
            reply,
            json!(omnect_device_service::put_restart_network(&socket_client).await),
        ),
    };

    if tx_result.send(result).is_err() {
        error!("handle_web_service_request: receiver dropped");
    }

    Ok(())
}

pub async fn run() -> Result<()> {
    let (tx_web_service, mut rx_web_service) = mpsc::channel(100);

    let mut signals = Signals::new(TERM_SIGNALS)?;

    let web_service = WebService::new(tx_web_service.clone());
    let socket = "/run/omnect-device-service/api.sock";
    let socket_client: SocketClient = SocketClient::open(socket);

    loop {
        select! (

            _ = signals.next() => {
                signals.handle().close();
                web_service.shutdown().await;
                return Ok(())
            },

            request = rx_web_service.recv() => {
                self::handle_web_service_request(request.unwrap(),&socket_client).await?
            }
        );
    }
}

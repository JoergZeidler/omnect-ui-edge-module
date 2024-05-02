use crate::backend::client::SocketClient;
use actix_web::web::Json;
use common::model::omnect_device_service::RebootResponse;
use common::model::omnect_device_service::ReloadNetworkResponse;
use common::model::omnect_device_service::VersionResponse;

pub async fn get_version(socket_client: &SocketClient) -> Json<VersionResponse> {
    match socket_client.get_version().await {
        // Err(error) => println!("{}", error),
        Err(_) => todo!(),
        Ok(value) => {
            println!("{:?}", value);
            Json(value)
        }
    }
}

pub async fn put_reboot(socket_client: &SocketClient) -> Json<RebootResponse> {
    match socket_client.put_reboot().await {
        // Err(error) => println!("{}", error),
        Err(_) => todo!(),
        Ok(value) => {
            println!("{:?}", value);
            Json(value)
        }
    }
}

pub async fn put_reload_network(socket_client: &SocketClient) -> Json<ReloadNetworkResponse> {
    match socket_client.put_reload_network().await {
        // Err(error) => println!("{}", error),
        Err(_) => todo!(),
        Ok(value) => {
            println!("{:?}", value);
            Json(value)
        }
    }
}

use actix_web::{get, put, web::Json};
use common::model::omnect_device_service::RebootResponse;
use common::model::omnect_device_service::RestartNetworkResponse;
use common::model::omnect_device_service::VersionResponse;
use reqwest;

#[get("/os-version")]
pub async fn get_version() -> Json<VersionResponse> {
    let client = reqwest::Client::new();
    let response: VersionResponse = client
        .get("http://127.0.0.1:8086/os-version")
        // confirm the request using send()
        .send()
        .await
        // the rest is the same!
        .unwrap()
        .json()
        .await
        .unwrap();

    Json(response)
}

#[put("/reboot")]
pub async fn put_reboot() -> Json<RebootResponse> {
    let client = reqwest::Client::new();
    let response: RebootResponse = client
        .put("http://127.0.0.1:8086/reboot")
        // confirm the request using send()
        .send()
        .await
        // the rest is the same!
        .unwrap()
        .json()
        .await
        .unwrap();

    Json(response)
}

#[put("/restart-network")]
pub async fn put_restart_network() -> Json<RestartNetworkResponse> {
    let client = reqwest::Client::new();
    let response: RestartNetworkResponse = client
        .put("http://127.0.0.1:8086/restart-network")
        // confirm the request using send()
        .send()
        .await
        // the rest is the same!
        .unwrap()
        .json()
        .await
        .unwrap();

    Json(response)
}

use actix_web::{get, web::Json};
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

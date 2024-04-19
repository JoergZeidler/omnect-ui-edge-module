use common::model::omnect_device_service::RebootResponse;
use common::model::omnect_device_service::RestartNetworkResponse;
use common::model::omnect_device_service::VersionResponse;
use reqwasm::http::Request;
use yew::prelude::*;

#[function_component(App)]
fn app_component() -> Html {
    let version_response = Box::new(use_state(|| None));
    let reboot_response = Box::new(use_state(|| None));
    let restart_network_response = Box::new(use_state(|| None));
    let error = Box::new(use_state(|| None));
    let error2 = Box::new(use_state(|| None));
    let error3 = Box::new(use_state(|| None));
    let endpoint_version = Box::new(format!("api/os-version"));
    let endpoint_reboot = Box::new(format!("api/reboot"));
    let endpoint_restart_network = Box::new(format!("api/restart-network"));

    let retry_reboot = {
        let reboot_response = reboot_response.clone();
        let error2 = error2.clone();
        let endpoint_reboot = endpoint_reboot.clone();
        Callback::from(move |_| {
            let reboot_response = reboot_response.clone();
            let error2 = error2.clone();
            let endpoint_reboot = endpoint_reboot.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_response = Request::put(&endpoint_reboot).send().await;
                match fetched_response {
                    Ok(response) => {
                        let json: Result<RebootResponse, _> = response.json().await;
                        match json {
                            Ok(f) => {
                                reboot_response.set(Some(f));
                            }
                            Err(e) => error2.set(Some(e.to_string())),
                        }
                    }
                    Err(e) => error2.set(Some(e.to_string())),
                }
            });
        })
    };

    let retry_version = {
        let version_response = version_response.clone();
        let error = error.clone();
        let endpoint_version = endpoint_version.clone();
        Callback::from(move |_| {
            let version_response = version_response.clone();
            let error = error.clone();
            let endpoint_version = endpoint_version.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_response = Request::get(&endpoint_version).send().await;
                match fetched_response {
                    Ok(response) => {
                        let json: Result<VersionResponse, _> = response.json().await;
                        match json {
                            Ok(f) => {
                                version_response.set(Some(f));
                            }
                            Err(e) => error.set(Some(e.to_string())),
                        }
                    }
                    Err(e) => error.set(Some(e.to_string())),
                }
            });
        })
    };

    let retry_restart_network = {
        let restart_network_response = restart_network_response.clone();
        let error3 = error3.clone();
        let endpoint_restart_network = endpoint_restart_network.clone();
        Callback::from(move |_| {
            let restart_network_response = restart_network_response.clone();
            let error3 = error3.clone();
            let endpoint_restart_network = endpoint_restart_network.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_response = Request::put(&endpoint_restart_network).send().await;
                match fetched_response {
                    Ok(response) => {
                        let json: Result<RestartNetworkResponse, _> = response.json().await;
                        match json {
                            Ok(f) => {
                                restart_network_response.set(Some(f));
                            }
                            Err(e) => error3.set(Some(e.to_string())),
                        }
                    }
                    Err(e) => error3.set(Some(e.to_string())),
                }
            });
        })
    };

    let new_osname = match (*version_response).as_ref() {
        Some(response) => response.version.osName.clone(),
        None => "unkwon".to_string(),
    };
    let new_swversion = match (*version_response).as_ref() {
        Some(response) => response.version.swVersion.clone(),
        None => "unkwon".to_string(),
    };

    let new_reboot = match (*reboot_response).as_ref() {
        Some(response) => response.result.clone(),
        None => false,
    };

    let new_restart_network = match (*restart_network_response).as_ref() {
        Some(response) => response.result.clone(),
        None => false,
    };

    html! {
        <>
            <button onclick={retry_version}>{"Get Version"}</button>
            <p> {"osName: "} {new_osname} </p>
            <p> {"swVersion: "} {new_swversion} </p>
            <button onclick={retry_reboot}>{"Reboot"}</button>
            <p> {"reboot state: "} {new_reboot} </p>
            <button onclick={retry_restart_network}>{"Restart Network"}</button>
            <p> {"restart network state: "} {new_restart_network} </p>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

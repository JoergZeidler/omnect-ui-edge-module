use common::model::omnect_device_service::VersionResponse;
use reqwasm::http::Request;
use yew::prelude::*;

#[function_component(App)]
fn app_component() -> Html {
    let version_response = Box::new(use_state(|| None));
    let error = Box::new(use_state(|| None));
    let endpoint_version = Box::new(format!("api/os-version"));

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

    match (*version_response).as_ref() {
        Some(response) => html! {
            <>
                <button onclick={retry_version}>{"Get Version"}</button>
                { response.version.clone() }
            </>
        },
        None => match (*error).as_ref() {
            Some(e) => {
                html! {
                    <>
                        {"error"} {e}
                        <button onclick={retry_version}>{"retry_version"}</button>
                    </>
                }
            }
            None => {
                html! {
                    <>
                        <button onclick={retry_version}>{"Get Version"}</button>
                    </>
                }
            }
        },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

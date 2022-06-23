use wasm_bindgen::JsValue;
use web_sys::console;
use yew::prelude::*;

struct ConstructionMessage {
    expected_end: String,
    title: String,
    message: String,
}

#[function_component(App)]
fn app() -> Html {
    let construction = ConstructionMessage {
        expected_end: "June 2022".to_string(),
        title: "Under Construction".to_string(),
        message: "Pardon our dust!".to_string(),
    };

    console::log_1(&JsValue::from_str("Hello from Rust!"));
    html! {
        <>
            <div align="center">
                <h1>{construction.title}</h1>
                <h3>{construction.message}</h3>
                <p>{ format!("Expected completion: {end}", end = construction.expected_end) }</p>
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}

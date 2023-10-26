use stylist::yew::styled_component;
use themer::yew::use_theme;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

use crate::{components::Spinner, style::themes::ThemeChoice};

pub const BORDER_WIDTH: &str = "2px";
pub const BORDER_RADIUS: &str = "5px";

#[derive(Properties, PartialEq)]
pub struct IFrameProps {
    pub class: Option<Classes>,
    pub src: AttrValue,
}

#[styled_component(IFrame)]
pub fn view(props: &IFrameProps) -> Html {
    let theme = use_theme::<ThemeChoice>();

    let ctr_css = css! {
        background-color: white;
        border-width: ${BORDER_WIDTH};
        border-style: solid;
        border-color: ${theme.color};
        border-radius: ${BORDER_RADIUS};
        overflow: hidden;

        /* For spinner centering */
        display: flex;
        justify-content: center;
        align-items: center;
        flex-direction: column;
    };

    let iframe_css = css! {
        border: 0;
        display:none;
        overflow: hidden;
        width: 100%;
        height: 100%;
    };

    let loading_handle = NodeRef::default();
    let show = Callback::from({
        let loading_handle = loading_handle.clone();
        move |e: Event| {
            if let Some(loading_panel) = loading_handle.cast::<Element>() {
                // Hide loading panel
                _ = loading_panel.set_attribute("style", "display: none");
                // Show iframe
                _ = e
                    .target()
                    .and_then(|t| t.dyn_into::<Element>().ok())
                    .map(|e| e.set_attribute("style", "display: block"));
            }
        }
    });

    html! {
        <div class={classes!(props.class.clone(), ctr_css)}>
            <div ref={loading_handle}>
                <Spinner />
                <br />
                <a href={ props.src.clone() }>
                {"Not loading?"}
                </a>
            </div>
            <iframe
                onload={ show }
                src={ props.src.clone() }
                class={iframe_css}
            />
        </div>
    }
}

use crate::components::Spinner;
use crate::style::themes::ThemeChoice;
use cssugar::prelude::*;
use stylist::yew::styled_component;
use themer::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

pub const BORDER_WIDTH: Length = Length::Px(2.0);
pub const BORDER_RADIUS: Length = Length::Px(5.0);

#[derive(Properties, PartialEq)]
pub struct IFrameProps {
    pub class: Option<Classes>,
    pub src: &'static str,
}

#[styled_component(IFrame)]
pub fn view(props: &IFrameProps) -> Html {
    let theme = use_theme::<ThemeChoice>();
    let border = theme.color;
    let border_bg = theme.color.alpha(0.15);
    let ctr_css = css! {
        background-color: ${border_bg};
        border-width: ${BORDER_WIDTH};
        border-style: solid;
        border-color: ${border};
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
            if let Some(loading_panel) = loading_handle
                .get()
                .and_then(|t| t.dyn_into::<Element>().ok())
            {
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
                <a href={ props.src }>
                {"Not loading?"}
                </a>
            </div>
            <iframe
                onload={ show }
                src={ props.src }
                class={iframe_css}
            />
        </div>
    }
}

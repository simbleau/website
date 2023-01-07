use crate::components::Spinner;
use crate::style::themes::BrandChoice;
use stylist::yew::styled_component;
use themer::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

const BORDER_WIDTH: &str = "2px";
const BORDER_RADIUS: &str = "5px";

#[derive(Properties, PartialEq)]
pub struct IFrameProps {
    pub class: Option<Classes>,
    pub src: &'static str,
}

#[styled_component(IFrame)]
pub fn view(props: &IFrameProps) -> Html {
    let theme = use_theme::<BrandChoice>();

    let style = css! {
        r#"
        #iframe-ctr {
            background-color: ${bg};
            height: 100%;
            width: 100%;
            border: ${bw} solid ${fg};
            border-radius: ${br};
            overflow: hidden;

            /* For spinner centering */
            display: flex;
            justify-content: center;
            align-items: center;
            flex-direction: column;
        }
        iframe {
            border: 0;
            display:none;
            overflow: hidden;
            width: 100%;
            height: 100%;
        }
        "#,
        fg = theme.color,
        bg = theme.color.alpha(0.15),
        bw = BORDER_WIDTH,
        br = BORDER_RADIUS,
    };

    let loading_handle = NodeRef::default();
    let show = Callback::from({
        let loading_handle = loading_handle.clone();
        move |e: Event| {
            let loader = loading_handle
                .get()
                .and_then(|t| t.dyn_into::<Element>().ok())
                .expect("Could not get loading handle");
            loader
                .set_attribute("style", "display: none")
                .expect("Could not hide loader");
            let iframe = e
                .target()
                .and_then(|t| t.dyn_into::<Element>().ok())
                .expect("Unable to get iFrame");
            iframe
                .set_attribute("style", "display: block")
                .expect("Could not show iframe");
        }
    });

    html! {
        <div align="center" class={classes!(props.class.clone(), style)}>
            <div id="iframe-ctr">
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
                    width="100%"
                    height="100%"
                />
            </div>
        </div>
    }
}

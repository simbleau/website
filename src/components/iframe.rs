use crate::components::Spinner;
use crate::style::themes::ThemeChoice;
use cssugar::prelude::*;
use stylist::yew::styled_component;
use themer::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

const BORDER_WIDTH: Length = Length::Px(2.0);
const BORDER_RADIUS: Length = Length::Px(5.0);

#[derive(Properties, PartialEq)]
pub struct IFrameProps {
    pub class: Option<Classes>,
    pub src: &'static str,
}

#[styled_component(IFrame)]
pub fn view(props: &IFrameProps) -> Html {
    let theme = use_theme::<ThemeChoice>();

    let style = css! {
        r#"
        #iframe-ctr {
            background-color: ${border_bg};
            height: 100%;
            width: 100%;
            border: ${border_width} solid ${border};
            border-radius: ${border_radius};
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
        border = theme.color,
        border_bg = theme.color.alpha(0.15),
        border_width = BORDER_WIDTH,
        border_radius = BORDER_RADIUS,
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

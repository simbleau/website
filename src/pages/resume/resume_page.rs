use log::info;
use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

use crate::components::{Hyperlink, Spinner, Url};
use crate::style::icons::{Icon, IconMask};
use crate::style::themes::use_theme;

const BORDER_WIDTH: &str = "2px";
const BORDER_RADIUS: &str = "5px";

#[styled_component(ResumePage)]
pub fn view() -> Html {
    let theme = use_theme();

    let style = css! {
        r#"
        #resume-ctr {
            background-color: ${bg};
            min-height: 500px;
            height: 75vh;
            max-height: 12in;
            border-top: ${bw} solid ${fg1};
            border-bottom: ${bw} solid ${fg1};
            border-left: 0;
            border-right: 0;
            width: 100%;

            display: flex;
            justify-content: center;
            align-items: center;
            flex-direction: column;
        }
        @media (min-width: calc(800px + ${bw} + ${bw})) {
            #resume-ctr {
                width: 800px;
                border: ${bw} solid ${fg1};
                border-radius: ${br};
            }
        }
        iframe {
            border: 0;
            display:none;
        }
        #text_wrap {
            margin: 5px;
            display: inline-block;
            vertical-align: middle;
            text-align: center;
        }
        "#,
        fg1 = theme.fg1,
        bg = theme.fg1.with_alpha(0.15),
        bw = BORDER_WIDTH,
        br = BORDER_RADIUS,
    };

    let loading_handle = NodeRef::default();
    let show_resume = Callback::from({
        let loading_handle = loading_handle.clone();
        move |e: Event| {
            let loader = loading_handle
                .get()
                .and_then(|t| t.dyn_into::<Element>().ok())
                .expect("Could not get loading handle");
            loader
                .set_attribute("style", "display: none")
                .expect("Could not hide loader");
            let resume = e
                .target()
                .and_then(|t| t.dyn_into::<Element>().ok())
                .expect("Unable to get iFrame");
            resume
                .set_attribute("style", "display: block")
                .expect("Could not show resume");
        }
    });

    html! {
        <div align="center" class={style}>
            <div id="text_wrap">
                <Icon
                    mask={IconMask::GitHub}
                    scale={1.5}
                    class={css!("margin-right: 3px;")}
                />
                {"This résumé is "}
                <Hyperlink
                    domain={Url::External("https://github.com/simbleau/resume")}
                    display={html!("source controlled") }
                />
                {" and "}
                <Hyperlink
                    domain={Url::External("https://github.com/simbleau/resume/actions")}
                    display={html!("automated") }
                />
            </div>
            <br />
            <div id="resume-ctr">
                <div ref={loading_handle}>
                    <Spinner />
                    <br />
                    <Hyperlink
                        domain={Url::External("https://github.com/simbleau/resume/releases/download/latest/resume.pdf")}
                        display={html!("Not loading?")}
                    />
                </div>
                <iframe
                    onload={show_resume}
                    src="https://docs.google.com/viewer?url=https://github.com/simbleau/resume/releases/download/latest/resume.pdf&embedded=true"
                    width="800"
                    height="500"
                />
            </div>
        </div>
    }
}

use crate::components::{Destination, Hyperlink, IconMask, Spinner};
use crate::style::themes::BrandChoice;
use cssugar::prelude::*;
use stylist::yew::styled_component;
use themer::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

const BORDER_WIDTH: Length = Length::Px(2.0);
const BORDER_RADIUS: Length = Length::Px(5.0);

#[styled_component(ResumePage)]
pub fn view() -> Html {
    let theme = use_theme::<BrandChoice>();
    let resume_bg = use_state(|| theme.color.alpha(0.15));

    let style = css! {
        r#"
        #resume-ctr {
            background-color: ${bg};
            min-height: 500px;
            height: 75vh;
            max-height: 12in;
            border-top: ${bw} solid ${border};
            border-bottom: ${bw} solid ${border};
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
                border: ${bw} solid ${border};
                border-radius: ${br};
            }
        }
        iframe {
            border: 0;
            display:none;
            width: 100%;
            height: 100%;
        }
        #text_wrap {
            margin: 5px;
            display: inline-block;
            vertical-align: middle;
            text-align: center;
        }
        "#,
        border = theme.color,
        bg = *resume_bg,
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
            resume_bg.set(WHITE);
        }
    });

    html! {
        <div align="center" class={style}>
            <div id="text_wrap">
                {"This résumé is "}
                <Hyperlink
                    icon={IconMask::GitHub}
                    to={Destination::External("https://github.com/simbleau/resume")}
                >
                    {"source controlled"}
                </Hyperlink>
                {" and "}
                <Hyperlink
                    to={Destination::External("https://github.com/simbleau/resume/actions")}
                >
                    {"automated"}
                </Hyperlink>
                {"."}
            </div>
            <br />
            <div id="resume-ctr">
                <div ref={loading_handle}>
                    <Spinner />
                    <br />
                    <Hyperlink
                        to={Destination::External("https://simbleau.github.io/resume")}
                    >
                        {"Not loading?"}
                    </Hyperlink>
                </div>
                <iframe
                    onload={show_resume}
                    src="https://simbleau.github.io/resume/embed.html"
                />
            </div>
        </div>
    }
}

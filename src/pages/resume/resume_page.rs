use stylist::yew::styled_component;
use yew::prelude::*;

use crate::components::{Hyperlink, Url};
use crate::style::icons::{Icon, IconMask};
use crate::style::themes::use_theme;

const BORDER_WIDTH: &str = "2px";
const BORDER_RADIUS: &str = "5px";

#[styled_component(ResumePage)]
pub fn view() -> Html {
    let theme = use_theme();

    let style = css! {
        r#"
        iframe {
            min-height: 500px;
            height: 75vh;
            max-height: 12in;
            border-top: ${bw} solid ${fg1};
            border-bottom: ${bw} solid ${fg1};
            border-left: 0;
            border-right: 0;
            width: 100%;
        }
        @media (min-width: calc(800px + ${bw} + ${bw})) {
            iframe {
                width: 800px;
                border: ${bw} solid ${fg1};
                border-radius: ${br};
            }
        }
        & > span {
            margin: 5px 0;
            display: inline-flex;
            align-items: center;
            vertical-align: middle;
        }
        & > span > * {
            margin: 0 5px;
        }
        "#,
        fg1 = theme.fg1,
        bw = BORDER_WIDTH,
        br = BORDER_RADIUS,
    };
    html! {
        <div align="center" class={style}>
            <span>
                <Icon
                    mask={IconMask::GitHub}
                    scale={1.5}
                />
                {" This résumé is "}
                <Hyperlink
                    domain={Url::External("https://github.com/simbleau/resume")}
                    display={html!("source controlled") }
                />
                {" and "}
                <Hyperlink
                    domain={Url::External("https://github.com/simbleau/resume/actions")}
                    display={html!("automated") }
                />
            </span>
            <br />
            <iframe
                src="https://docs.google.com/viewer?url=https://github.com/simbleau/resume/releases/download/latest/resume.pdf&embedded=true"
                width="800"
                height="500"
            />
        </div>
    }
}

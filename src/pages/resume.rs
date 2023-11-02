use crate::components::{ExternalLink, IFrame, IconMask};
use stylist::yew::styled_component;
use yew::prelude::*;

const PDF_MIN_HEIGHT: &str = "500px";
const PDF_HEIGHT: &str = "100vh";
const PDF_HEIGHT_PADDING: &str = "220px";
const PDF_MAX_HEIGHT: &str = "11in";
const PDF_WIDTH: &str = "100vw";
const PDF_MAX_WIDTH: &str = "8.5in";
const IFRAME_BORDER_WIDTH: &str = "2px";

#[styled_component]
pub fn ResumePage() -> Html {
    let resume_css = css! {
        r#"
            @media not screen and (min-width: ${min_width}) {
                border-left: 0 !important;
                border-right: 0 !important;
                border-radius: 0 !important;
            }
            width: ${PDF_WIDTH};
            max-width: ${PDF_MAX_WIDTH};
            min-height: ${PDF_MIN_HEIGHT};
            height: ${PDF_HEIGHT};
            max-height: ${PDF_MAX_HEIGHT};
        "#,
        min_width = format!("calc({} + {} + {})", PDF_MAX_WIDTH, IFRAME_BORDER_WIDTH, IFRAME_BORDER_WIDTH),
        PDF_WIDTH = PDF_WIDTH,
        PDF_MAX_WIDTH = PDF_MAX_WIDTH,
        PDF_MIN_HEIGHT = PDF_MIN_HEIGHT,
        PDF_HEIGHT = format!("calc({} - {})", PDF_HEIGHT, PDF_HEIGHT_PADDING),
        PDF_MAX_HEIGHT = PDF_MAX_HEIGHT,
    };

    html! {
        <>
            <div>
                {"this résumé is "}
                <ExternalLink
                    icon={IconMask::Git}
                    to={"https://github.com/simbleau/resume"}
                >
                    {"source controlled"}
                </ExternalLink>
                {" and "}
                <ExternalLink
                    icon={IconMask::Robot}
                    to={"https://github.com/simbleau/resume/actions"}
                >
                    {"automated"}
                </ExternalLink>
            </div>
            <br />
            <IFrame
                class={classes!(resume_css)}
                src="https://simbleau.github.io/resume/embed.html"
            />
            <br />
            <div>
                <ExternalLink
                    to={"https://github.com/simbleau/resume/releases/download/latest/resume.pdf"}
                >
                    {"click here"}
                </ExternalLink>
                {" to download"}
            </div>
        </>
    }
}

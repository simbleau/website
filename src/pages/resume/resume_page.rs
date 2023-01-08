use crate::components::{
    Destination, Hyperlink, IFrame, IconMask, IFRAME_BORDER_WIDTH,
};
use cssugar::prelude::*;
use stylist::yew::styled_component;
use yew::prelude::*;

const PDF_MIN_HEIGHT: Length = Length::Px(500.0);
const PDF_HEIGHT: Length = Length::Vh(100.0);
const PDF_HEIGHT_PADDING: Length = Length::Px(250.0);
const PDF_MAX_HEIGHT: Length = Length::In(12.0);
const PDF_WIDTH: Length = Length::Vw(100.0);
const PDF_MAX_WIDTH: Length = Length::Px(800.0);

#[styled_component(ResumePage)]
pub fn view() -> Html {
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
        min_width = PDF_MAX_WIDTH + IFRAME_BORDER_WIDTH + IFRAME_BORDER_WIDTH,
        PDF_WIDTH = PDF_WIDTH,
        PDF_MAX_WIDTH = PDF_MAX_WIDTH,
        PDF_MIN_HEIGHT = PDF_MIN_HEIGHT,
        PDF_HEIGHT = PDF_HEIGHT - PDF_HEIGHT_PADDING,
        PDF_MAX_HEIGHT = PDF_MAX_HEIGHT,
    };

    html! {
        <div align="center">
            <div id="text_wrap">
                {"this résumé is "}
                <Hyperlink
                    icon={IconMask::Git}
                    to={Destination::External("https://github.com/simbleau/resume")}
                >
                    {"source controlled"}
                </Hyperlink>
                {" and "}
                <Hyperlink
                    icon={IconMask::Robot}
                    to={Destination::External("https://github.com/simbleau/resume/actions")}
                >
                    {"automated"}
                </Hyperlink>
            </div>
            <br />
            <IFrame
                class={classes!(resume_css)}
                src="https://simbleau.github.io/resume/embed.html"
            />
        </div>
    }
}

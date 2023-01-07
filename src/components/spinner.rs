use stylist::yew::styled_component;
use themer::prelude::*;
use yew::prelude::*;

use crate::style::themes::BrandChoice;

const R: f32 = 10.0;
const STROKE_WIDTH: f32 = 3.0;
pub const SPINNER_SIZE: f32 = 50.0;

#[styled_component(Spinner)]
pub fn spinner() -> Html {
    let theme = use_theme::<BrandChoice>();

    let spinner = css! {
        r#"
        & {
            align-items:center;
            vertical-align:middle;
            display:flex;
            justify-content:center;
            overflow:visible;
        }
        "#
    };

    let spinner_animation = css! {
        r#"
        & {
            animation:pt-spinner-animation 500ms linear infinite;
        }
        @keyframes pt-spinner-animation{
            from{
                transform:rotate(0deg);
            }
            to{
                transform:rotate(360deg);
            }
        }

        & > svg {
            display: block;
        }
        & > svg path {
            fill-opacity: 0;
        }
        & > svg > #track {
            stroke: ${stroke};
        }
        "#,
        stroke = theme.color,
    };

    let view_box = {
        let radius = R + STROKE_WIDTH / 2.00;
        let view_box_x = 50.00 - radius;
        let view_box_width = radius * 2.00;
        format!(
            "{view_box_x:.2} {view_box_x:.2} {view_box_width:.2} {view_box_width:.2}"
        )
    };
    let spinner_track = format!(
        "M 50,50 m 0,-{R:.0} a {R:.0},{R:.0} 0 1 1 0,{R2:.0} a {R:.0},{R:.0} 0 1 1 0,-{R2:.0}",
        R = R,
        R2 = R * 2.0,
    );

    html! {
        <div class={spinner}>
            <div class={spinner_animation}>
                <svg
                    width={SPINNER_SIZE.to_string()}
                    height={SPINNER_SIZE.to_string()}
                    stroke-width={STROKE_WIDTH.to_string()}
                    viewBox={view_box}
                >
                    <path
                        id="track"
                        d={spinner_track}
                        pathLength={"1.0"}
                        stroke-dasharray={"0.25 1.0"}
                        stroke-dashoffset={0.75}
                    />
                </svg>
            </div>
        </div>
    }
}

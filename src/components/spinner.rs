use stylist::yew::styled_component;
use yew::prelude::*;

use crate::style::themes::use_theme;

const R: f32 = 45.0;
const PATH_LENGTH: f32 = 280.0;
const SPINNER_MIN_SIZE: f32 = 10.0;
const STROKE_WIDTH: f32 = 4.0;
const MIN_STROKE_WIDTH: f32 = 16.0;
pub const SPINNER_SIZE_STANDARD: f32 = 50.0;

#[derive(Clone, PartialEq, Properties)]
pub struct SpinnerProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(SPINNER_SIZE_STANDARD)]
    pub size: f32,
    #[prop_or(0.25)]
    pub value: f32,
}

#[styled_component(Spinner)]
pub fn spinner(props: &SpinnerProps) -> Html {
    let theme = use_theme();

    let spinner = css! {
        r#"
        & {
            -webkit-box-align:center;
            -ms-flex-align:center;
            align-items:center;
            display:-webkit-box;
            display:-ms-flexbox;
            display:flex;
            -webkit-box-pack:center;
            -ms-flex-pack:center;
            justify-content:center;
            overflow:visible;
            vertical-align:middle;
        }
        "#
    };

    let spinner_animation = css! {
        r#"
        & {
            -webkit-animation:pt-spinner-animation 500ms linear infinite;
            animation:pt-spinner-animation 500ms linear infinite;
        }

        @keyframes pt-spinner-animation{
            from{
                -webkit-transform:rotate(0deg);
                        transform:rotate(0deg);
            }
            to{
                -webkit-transform:rotate(360deg);
                        transform:rotate(360deg);
            }
        }

        & > svg {
            display: block;
        }

        & > svg path {
            fill-opacity: 0;
        }

        & > svg > #head {
            stroke: ${bg2};
        }

        & > svg > #track {
            stroke: ${fg1};
        }
        "#,
        bg2 = theme.bg2,
        fg1 = theme.fg1,
    };

    let size = f32::max(SPINNER_MIN_SIZE, props.size);
    let stroke_width =
        f32::min(MIN_STROKE_WIDTH, (STROKE_WIDTH * 100.0) / size);
    let view_box = {
        let radius = R + stroke_width / 2.00;
        let view_box_x = 50.00 - radius;
        let view_box_width = radius * 2.00;
        format!(
            "{:.2} {:.2} {:.2} {:.2}",
            view_box_x, view_box_x, view_box_width, view_box_width,
        )
    };
    let spinner_track = format!(
        "M 50,50 m 0,-{R:.0} a {R:.0},{R:.0} 0 1 1 0,{R2:.0} a {R:.0},{R:.0} 0 1 1 0,-{R2:.0}",
        R = R,
        R2 = R * 2.0,
    );
    let stroke_offset = PATH_LENGTH - PATH_LENGTH * props.value.clamp(0.0, 1.0);

    html! {
        <div
            class={classes!(
                spinner,
                props.class.clone(),
            )}
        >
            <div
                class={classes!(spinner_animation)}
            >
                <svg
                    width={size.to_string()}
                    height={size.to_string()}
                    stroke-width={stroke_width.to_string()}
                    viewBox={view_box}
                >
                    <path
                        id="head"
                        d={spinner_track.clone()}
                    />
                    <path
                        id="track"
                        d={spinner_track}
                        pathLength={PATH_LENGTH.to_string()}
                        stroke-dasharray={format!("{} {}", PATH_LENGTH, PATH_LENGTH)}
                        stroke-dashoffset={stroke_offset.to_string()}
                    />
                </svg>
            </div>
        </div>
    }
}

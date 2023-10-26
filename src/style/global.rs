use super::themes::ThemeChoice;
use stylist::{css, StyleSource};
use themer::yew::use_theme;
use yew::hook;

#[hook]
pub fn use_global_css() -> StyleSource {
    let theme = use_theme::<ThemeChoice>();
    css!(
        r#"
            html, body {
                /* General styling */
                padding: 0;
                margin: 0;
                position: relative;
                min-height: 100vh;
                scroll-behavior: smooth;

                /* Font sizing */
                font-weight: ${fw};
                font-size: ${fs};

                /* Theme Application */
                transition-property: background-color, font-size, width, height;
                transition-duration: 250ms;
                background-color: ${background_color};
                color: ${color};
            }

            @media (min-width: 768px) {
                html, body {
                    font-size: ${fsm};
                }
            }
            @media (min-width: 992px) {
                html, body {
                    font-size: ${fst};
                }
            }
            @media (min-width: 1200px) {
                html, body {
                    font-size: ${fsd};
                }
            }

            /* Headers */
            h1,
            h2,
            h3,
            h4,
            h5,
            h6 {
                color: ${header_color};
                font-weight: ${fwh};
            }

            /* Links */
            a {
                color: ${link};
                text-decoration:none;
            }
            a:hover {
                color: ${link_hover};
                text-decoration: underline;
            }

            h1 a,
            h2 a,
            h1 a:hover,
            h2 a:hover {
                text-decoration:none;
            }

            /* Fonts */
            html,
            body {
                text-rendering: optimizeLegibility;
                font-family: ${body_font};
            }

            h1,
            h2,
            h3,
            h4,
            h5,
            h6 {
                font-family: ${header_font};
            }

            .preformatted {
                font-family: ${mono_font};
            }
        "#,
        color = theme.color,
        background_color = theme.background_color,
        link = theme.link,
        link_hover = theme.link_hover,
        header_color = theme.header_color,
        body_font = theme.body_font,
        header_font = theme.header_font,
        mono_font = theme.mono_font,
        fs = theme.fs,
        fsm = theme.fsm,
        fst = theme.fst,
        fsd = theme.fsd,
        fw = theme.fw,
        fwh = theme.fwh,
    )
}

use stylist::{css, StyleSource};

use crate::style::themes::use_theme;
pub const THEME_TRANSITION_SPEED: &str = "0.5s";

pub fn css() -> StyleSource<'static> {
    let theme = use_theme();
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
                transition: background-color ${transition_speed},
                            font-size ${transition_speed};
                            width: ${transition_speed};
                            height: ${transition_speed};
                background-color: ${bg1};
                color: ${fg1};
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
                color: ${fg2};
                font-weight: ${fwh};
            }

            /* Links */
            a {
                color: ${ac1};
                text-decoration:none;
            }
            a:hover {
                color: ${ac2};
                text-decoration:underline;
            }

            h1 a,
            h2 a,
            main nav a,
            h1 a:hover,
            h2 a:hover,
            main nav a:hover {
                text-decoration:none
            }
        "#,
        bg1 = theme.bg1,
        fg1 = theme.fg1,
        fg2 = theme.fg2,
        ac1 = theme.ac1,
        ac2 = theme.ac2,
        fs = theme.fs,
        fsm = theme.fsm,
        fst = theme.fst,
        fsd = theme.fsd,
        fw = theme.fw,
        fwh = theme.fwh,
        transition_speed = THEME_TRANSITION_SPEED,
    )
}

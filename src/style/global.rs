use super::themes::ThemeChoice;
use stylist::{css, yew::use_media_query, StyleSource};
use themer::yew::use_theme;
use yew::hook;

#[hook]
pub fn use_global_css() -> (StyleSource, StyleSource) {
    let theme = use_theme::<ThemeChoice>();

    let is_mobile = use_media_query("(max-width: 992px)");
    let is_tablet = use_media_query("(max-width: 1200px)");

    // FIXME: after https://github.com/futursolo/stylist-rs/issues/147
    let buggy_css = css! {
        r#"
            a:hover {
                color: ${link_hover};
                text-decoration: underline;
            }
        "#,
        link_hover = theme.link_hover.display_rgb()
    };

    let global_css = css! {
        /* General styling */
        padding: 0;
        margin: 0;
        position: relative;
        min-height: 100vh;
        scroll-behavior: smooth;

        /* Font sizing */
        font-weight: ${theme.fw};
        font-size: ${
            if is_mobile {
                theme.fsm
            } else if is_tablet {
                theme.fst
            }else {
                theme.fsd
            }
        };

        /* Theme Application */
        transition-property: background-color, font-size, width, height;
        transition-duration: 250ms;
        background-color: ${theme.background_color.display_rgb()};
        color: ${theme.color.display_rgb()};

        /* Headers */
        h1,
        h2,
        h3,
        h4,
        h5,
        h6 {
            font-weight: ${theme.fwh};
        }

        /* Links */
        a {
            color: ${theme.link.display_rgb()};
            text-decoration:none;
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
            font-family: ${theme.body_font};
        }

        .preformatted {
            font-family: ${theme.mono_font};
        }
    };

    (global_css, buggy_css)
}

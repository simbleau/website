use stylist::{css, StyleSource};

use crate::style::use_theme;

pub fn css() -> StyleSource<'static> {
    let theme = use_theme();
    css!(
        r#"
            :root {
                --fs: 1rem;
                --fw: 400;
                --fh: 700;
                --bg: ${bg};
                --fg: ${fg};
            }
            @media (min-width: 768px) {
                :root {
                    --fs: 1.04rem;
                }
            }
            @media (min-width: 992px) {
                :root {
                    --fs: 1.08rem;
                }
            }
            @media (min-width: 1200px) {
                :root {
                    --fs: 1.12rem;
                }
            }

            html, body {
                /* General styling */
                padding: 0;
                margin: 0;
                position: relative;
                min-height: 100vh;
                scroll-behavior: smooth;

                /* Font sizing */
                font-weight:var(--fw);
                font-size:var(--fs);

                /* Theme Application */
                background-color: var(--bg);
                color: var(--fg);
            }

            /* Header sizing */
            h1,
            h2,
            h3,
            h4,
            h5,
            h6 {
                font-weight:var(--fh);
            }

            /* Link Text Decoration */
            a {
                text-decoration:none
            }
            a:hover {
                text-decoration:underline
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
        bg = theme.bg1.to_css(),
        fg = theme.fg1.to_css(),
    )
}

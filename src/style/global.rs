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
                --bg1: ${bg1};
                --bg2: ${bg2};
                --fg1: ${fg1};
                --fg2: ${fg2};
                --ac1: ${ac1};
                --ac2: ${ac2};
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
                background-color: var(--bg1);
                color: var(--fg1);
            }

            /* Headers */
            h1,
            h2,
            h3,
            h4,
            h5,
            h6 {
                color: var(--fg2);
                font-weight:var(--fh);
            }

            /* Links */
            a {
                color: var(--ac1);
                text-decoration:none;
            }
            a:hover {
                color: var(--ac2);
                text-decoration:underline;
            }

            /* Specific styling (Because I feel like it!) */
            h1 a,
            h2 a,
            main nav a,
            h1 a:hover,
            h2 a:hover,
            main nav a:hover {
                text-decoration:none
            }
        "#,
        bg1 = theme.bg1.to_css(),
        bg2 = theme.bg2.to_css(),
        fg1 = theme.fg1.to_css(),
        fg2 = theme.fg2.to_css(),
        ac1 = theme.ac1.to_css(),
        ac2 = theme.ac2.to_css(),
    )
}

use stylist::yew::styled_component;
use yew::prelude::*;

pub const FOOTER_HEIGHT: &str = "150px";
pub const FOOTER_PADDING: &str = "5px";

#[styled_component(Footer)]
pub fn footer() -> Html {
    let css = css!(
        r#"
            /* Anchor to bottom */
            position: absolute;
            bottom: 0;
            width: 100%;
            height: ${footer_height};

            /* Center footer line */
            display: flex;
            justify-content: center;
            align-items: center;

            /* Styling */
            padding-top: ${footer_padding};
            padding-bottom: ${footer_padding};
        "#,
        footer_height = FOOTER_HEIGHT,
        footer_padding = FOOTER_PADDING,
    );

    html! {
        <footer class={ css }>
            <div id="footer_wrap" align="center">
                <p>
                    { "2022 " }
                    <i class="i-copyright"></i>
                    { " Spencer C. Imbleau" }
                </p>
                { "Built with " }<i class="i-coffee"></i>
                {" and a "}
                <i class="i-keyboard"></i>
                {" using " }
                <i class="b-rust"></i>
                <br />
                <small>
                    <a href="https://github.com/simbleau/website">
                        <i class="i-edit"></i>
                        {" Edit on GitHub"}
                    </a>
                </small>
            </div>
        </footer>
    }
}

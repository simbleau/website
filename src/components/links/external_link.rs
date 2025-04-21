use crate::{
    components::{Icon, IconMask},
    hooks::use_theme,
};
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ExternalLinkProps {
    pub to: AttrValue,
    #[prop_or_default]
    pub target: Option<AttrValue>,
    #[prop_or_default]
    pub download: Option<AttrValue>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub icon: Option<IconMask>,
    #[prop_or_default]
    pub class: Classes,
}

#[styled_component(ExternalLink)]
pub fn view(props: &ExternalLinkProps) -> Html {
    let theme = use_theme();

    let hitbox_style = css! {
        & {
            display: inline-block;
        }
        & *[data-aui-id="linkicon"] {
            background-color: ${theme.link.display_rgb()};
        }
        &:hover *[data-aui-id="linkicon"] {
            background-color: ${theme.link_hover.display_rgb()};
        }
    };

    let link_css = css! {
        display: flex;
        flex-direction: row;
        align-items: baseline;
    };

    html! {
        <div class={classes!(hitbox_style, props.class.clone())}>
            <a href={ props.to.clone() } class={link_css} target={props.target.clone()} download={props.download.clone()}>
                if let Some(mask) = props.icon {
                    <Icon
                        data_aui_id="linkicon"
                        mask={mask}
                        class={classes!(css!("align-self: middle; margin-right: 3px;"))}
                    />
                }
                { props.children.clone() }
                if props.download.is_some() {
                    <Icon
                        data_aui_id="linkicon"
                        mask={ IconMask::Download }
                        class={classes!(css!("align-self: baseline; margin-left: 3px;"))}
                    />
                } else {
                    <Icon
                        data_aui_id="linkicon"
                        mask={ IconMask::Share }
                        class={classes!(css!("align-self: baseline; margin-left: 3px;"))}
                    />
                }
            </a>
        </div>
    }
}

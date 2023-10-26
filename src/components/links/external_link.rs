use crate::{
    components::{Icon, IconMask},
    style::themes::ThemeChoice,
};
use stylist::yew::styled_component;
use themer::yew::use_theme;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ExternalLinkProps {
    #[prop_or(AttrValue::Static("#"))]
    pub to: AttrValue,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub icon: Option<IconMask>,
    #[prop_or_default]
    pub class: Classes,
}

#[styled_component(ExternalLink)]
pub fn view(props: &ExternalLinkProps) -> Html {
    let theme = use_theme::<ThemeChoice>();

    let hitbox_style = css! {
        & {
            display: inline-block;
        }
        & *[data-aui-id="linkicon"] {
            background-color: ${theme.link};
        }
        &:hover *[data-aui-id="linkicon"] {
            background-color: ${theme.link_hover};
        }
    };

    let link_css = css! {
        display: flex;
        flex-direction: row;
        align-items: baseline;
    };

    html! {
        <div class={classes!(hitbox_style, props.class.clone())}>
            <a href={ props.to.clone() } class={link_css} >
                if let Some(mask) = props.icon {
                    <Icon
                        data_aui_id="linkicon"
                        mask={mask}
                        class={classes!(css!("align-self: middle; margin-right: 3px;"))}
                    />
                }
                { props.children.clone() }
                <Icon
                    data_aui_id="linkicon"
                    mask={ IconMask::Share }
                    class={classes!(css!("align-self: baseline; margin-left: 3px;"))}
                    scale={ 0.75 }
                />
            </a>
        </div>
    }
}

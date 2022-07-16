use stylist::css;
use yew::prelude::*;

use crate::style::themes::use_theme;

#[derive(Clone, PartialEq, Properties)]
pub struct DividerProps {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Divider)]
pub fn view(props: &DividerProps) -> Html {
    let theme = use_theme();

    let style = css! {
        r#"
            & {
                border-bottom: 1px solid ${ac1};
                border-right: 1px solid ${ac1};
                margin: 5px;
            }
        "#,
        ac1 = theme.ac1,
    };

    html! {
        <span
            class={classes!(
                style,
                props.class.clone(),
            )}
        />
    }
}

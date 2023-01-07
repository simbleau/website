use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum CalloutIntent {
    Primary,
    Success,
    Warning,
    Danger,
}

#[derive(Clone, PartialEq, Properties)]
pub struct CalloutProps {
    #[prop_or_default]
    pub class: Classes,
    pub intent: CalloutIntent,
    pub children: html::Children,
}

#[styled_component(Callout)]
pub fn callout(props: &CalloutProps) -> Html {
    let intent_css = match props.intent {
        CalloutIntent::Primary => css! {
            background-color: #0d6efd20;
        },
        CalloutIntent::Success => css! {
            background-color: #19875420;
        },
        CalloutIntent::Warning => css! {
            background-color: #ffc10720;
        },
        CalloutIntent::Danger => css! {
            background-color: #dc354520;
        },
    };

    let card_css = css! {
        position: relative;
        box-shadow: 0px 1px;
        border-radius: 5px;
        padding: 0;
        margin: 0;
        width: 100%;
    };

    let classes = classes!(props.class.clone(), intent_css, card_css);
    html! {
        <div class={classes} >
            <div id="card-children" style="padding: 10px; margin: 0;">
                { props.children.clone() }
            </div>
        </div>
    }
}

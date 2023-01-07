use gloo_timers::callback::Timeout;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct TransitionProps {
    pub time: u32,
    pub src: &'static str,
}

#[styled_component(Transition)]
pub fn view(props: &TransitionProps) -> Html {
    let transition = css!(
        r#"
            position: absolute;
            display: flex;
            justify-content: center;
            align-items: center;
            width: auto;
            height: calc(100vh - 90px);
            pointer-events: none;
        "#,
    );

    let transition_ref = use_node_ref();
    let callback = {
        let transition_ref = transition_ref.clone();
        let animation_time_ms = props.time;
        Callback::from(move |_| {
            let transition = transition_ref
                .get()
                .expect("Transition could not be found!");
            Timeout::new(animation_time_ms, move || {
                if let Some(parent) = transition.parent_node() {
                    parent
                        .remove_child(&transition)
                        .expect("Could not remove transition!");
                }
            })
            .forget();
        })
    };

    html! {
        <object
            onload={ callback }
            class={ transition }
            ref={ transition_ref }
            type="image/svg+xml"
            data={ props.src }
        />
    }
}

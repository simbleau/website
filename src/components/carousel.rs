use std::{cell::RefCell, rc::Rc};

use gloo_timers::callback::{Interval, Timeout};
use gloo_utils::window;
use stylist::yew::styled_component;
use web_sys::HtmlElement;
use yew::{prelude::*, virtual_dom::AttrValue};

#[derive(Properties, PartialEq)]
pub struct CarouselProps {
    pub images: Vec<&'static str>,
    #[prop_or(AttrValue::Static("700px"))]
    pub width: AttrValue,
    #[prop_or(AttrValue::Static("300px"))]
    pub height: AttrValue,
    #[prop_or(3000)]
    pub switch_ms: u32,
    #[prop_or(500)]
    pub transition_ms: u32,
    #[prop_or_default]
    pub img_class: Option<Classes>,
    #[prop_or_default]
    pub class: Option<Classes>,
}

fn show(node: &HtmlElement) {
    node.set_attribute("style", "opacity: 1").unwrap();
}
fn hide(node: &HtmlElement) {
    node.set_attribute("style", "opacity: 0").unwrap();
}

#[styled_component(Carousel)]
pub fn view(props: &CarouselProps) -> Html {
    let img_refs = Rc::new(
        props
            .images
            .iter()
            .map(|_| NodeRef::default())
            .collect::<Vec<NodeRef>>(),
    );

    {
        let switch_time = props.switch_ms;
        let carousel_time = props.switch_ms * img_refs.len() as u32;
        let transition_time = props.transition_ms;
        let img_refs = img_refs.clone();
        use_effect(move || {
            let timeouts = Rc::new(RefCell::new(vec![]));
            let intervals = Rc::new(RefCell::new(vec![]));

            if img_refs.len() >= 2 {
                for (i, img_node) in img_refs
                    .iter()
                    .map(|img_ref| img_ref.cast::<HtmlElement>())
                    .enumerate()
                {
                    if let Some(img) = img_node {
                        let offset_start_time = i as u32 * switch_time;

                        // First show/hide
                        timeouts.borrow_mut().push(
                            // Delay first animation by position in vector
                            Timeout::new(offset_start_time, {
                                let img = img.clone();
                                move || show(&img)
                            })
                            .forget(),
                        );
                        timeouts.borrow_mut().push(
                            // Delay first animation by position in vector
                            Timeout::new(
                                offset_start_time
                                    + switch_time
                                    + transition_time,
                                {
                                    let img = img.clone();
                                    move || hide(&img)
                                },
                            )
                            .forget(),
                        );

                        // Routine show/hide
                        timeouts.borrow_mut().push(
                            // Delay first animation by position in vector
                            Timeout::new(offset_start_time, {
                                let timeouts = timeouts.clone();
                                let intervals = intervals.clone();
                                move || {
                                    // Show interval
                                    intervals.borrow_mut().push(
                                        Interval::new(carousel_time, {
                                            let timeouts = timeouts.clone();
                                            move || {
                                                show(&img);
                                                let img = img.clone();
                                                timeouts.borrow_mut().push(
                                                    Timeout::new(
                                                        switch_time
                                                            + transition_time,
                                                        move || hide(&img),
                                                    )
                                                    .forget(),
                                                );
                                            }
                                        })
                                        .forget(),
                                    );
                                }
                            })
                            .forget(),
                        );
                    }
                }
            }

            // No longer being rendered - clear all timers
            move || {
                for t in timeouts.borrow().iter() {
                    window().clear_timeout_with_handle(*t);
                }
                for i in intervals.borrow().iter() {
                    window().clear_interval_with_handle(*i);
                }
            }
        });
    }

    let carousel_css = css! {
        r#"
            position: relative;
            width: ${w};
            height: ${h};
        "#,
        w = props.width.clone(),
        h = props.height.clone(),
    };

    let img_css = css! {
        r#"
            position: absolute;
            left: 0;
            top: 0;
            width: ${w};
            height: ${h};
            object-fit: cover;
            transition: opacity ${transition}ms;
        "#,
        w = props.width.clone(),
        h = props.height.clone(),
        transition = props.transition_ms,
    };

    let images = props
        .images
        .iter()
        .enumerate()
        .map(|(i, img_src)| {
            html! {
                <img
                    id={ format!("img-{i}") }
                    src={ *img_src }
                    alt=""
                    class={ classes!(img_css.clone(), props.img_class.clone()) }
                    style={ if i == 0 { "opacity: 1"} else { "opacity: 0" } }
                    ref={ &img_refs[i] }
                />
            }
        })
        .collect::<Html>();

    html! {
        <div id="carousel"
            class={classes!(carousel_css, props.class.clone())}>
            { images }
        </div>
    }
}

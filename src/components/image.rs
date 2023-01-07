use crate::style::themes::BrandChoice;
use crate::style::BRAND_COLOR;
use stylist::yew::styled_component;
use themer::prelude::*;
use yew::{prelude::*, virtual_dom::AttrValue};

#[derive(Properties, PartialEq)]
pub struct ImageProps {
    pub src: &'static str,
    #[prop_or_default]
    pub hover_src: Option<&'static str>,
    #[prop_or(AttrValue::Static("300px"))]
    pub width: AttrValue,
    #[prop_or(AttrValue::Static("300px"))]
    pub height: AttrValue,
    #[prop_or(AttrValue::Static("600px"))]
    pub max_width: AttrValue,
    #[prop_or(AttrValue::Static("600px"))]
    pub max_height: AttrValue,
}

#[styled_component(Image)]
pub fn view(props: &ImageProps) -> Html {
    let theme = use_theme::<BrandChoice>();

    let shape_css = css! {
        & {
            display: block;
            border-radius: 50%;
            width: ${props.width.clone()};
            max-width: ${props.max_width.clone()};
            height: ${props.height.clone()};
            max-height: ${props.max_height.clone()};
            object-fit: scale-down;
        }
    };

    let ctr_css = css! {
        r#"
        & {
            background:
                linear-gradient( white, white ) padding-box,
                linear-gradient(to bottom, ${brand}, ${bg} ) border-box;
            border: 2px solid transparent;
            box-shadow: 0 0 15px ${bg};

            transition-property: border, background, box-shadow, width, height;
            transition-duration: 0.5s;
        }

        &:hover {
            border: 6px solid transparent;
            box-shadow: 0 0 25px ${link};
        }
        "#,
        bg = theme.color.alpha(0.2),
        link = theme.link.alpha(0.5),
        brand = BRAND_COLOR,
    };

    let hover_css = css! {
        &:hover #hover {
            opacity: 1;
        }
        &:hover #fg {
            opacity: 0;
        }
    };

    let src_image_css = css! {
        & {
            transition: opacity 0.5s ease;
            opacity: 1;
        }
    };

    let hover_image_css = css! {
        & {
            transition: opacity 0.5s ease;
            opacity: 0.15;
        }
    };

    html! {
        <div
            class={
                if props.hover_src.is_some() {
                    classes!(shape_css.clone(), ctr_css, hover_css)
                } else {
                    classes!(shape_css.clone(), ctr_css)
                }
            }
            style="position: relative;"
        >
            if let Some(hover_src) = props.hover_src {
                <img
                    id="hover"
                    alt="Spencer C. Imbleau"
                    width={props.width.clone()}
                    height={props.height.clone()}
                    style="position: absolute; left: 0;"
                    class={ classes!(shape_css.clone(), hover_image_css) }
                    src={ hover_src }
                />
            }
            <img
                id="fg"
                alt="Spencer C. Imbleau"
                width={props.width.clone()}
                height={props.height.clone()}
                style="position: absolute; left: 0;"
                class={ classes!(shape_css, src_image_css) }
                src={ props.src }
            />
        </div>
    }
}

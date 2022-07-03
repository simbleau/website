use std::collections::HashMap;
use web_sys::HtmlInputElement;
use yew::{html, Component, Context, Html, NodeRef};

pub enum Event {
    BtnClick,
    TxtHover(&'static str),
}

pub struct ContactForm {
    refs: HashMap<&'static str, NodeRef>,
}

impl Component for ContactForm {
    type Message = Event;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut refs = HashMap::new();
        refs.insert("name", NodeRef::default());
        refs.insert("message", NodeRef::default());
        Self { refs }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Event::BtnClick => {
                let name = &self
                    .refs
                    .get("name")
                    .unwrap()
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value();
                let message = &self
                    .refs
                    .get("message")
                    .unwrap()
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value();

                log::debug!("Name: {}, Message: {}", name, message);
                false
            }
            Event::TxtHover(key) => {
                self.refs
                    .get(key)
                    .unwrap()
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .focus()
                    .unwrap();
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Event::BtnClick);
        html! {
            <div class="input-container">
            <label>{ "Name" }</label>
            <input
                onmouseover={ ctx.link().callback(|_| Event::TxtHover("name")) }
                ref={self.refs.get("name").unwrap().clone()}
                type="text"
                placeholder="name"
            />
            <br/>
            <label>{ "Message" }</label>
            <input
                onmouseover={ ctx.link().callback(|_| Event::TxtHover("message")) }
                ref={self.refs.get("message").unwrap().clone()}
                type="text"
                placeholder="message"
            />
            <br/>
            <button {onclick}>{ "Log - Test" }</button>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.refs
                .get("name")
                .unwrap()
                .cast::<HtmlInputElement>()
                .unwrap()
                .focus()
                .unwrap();
        }
    }
}

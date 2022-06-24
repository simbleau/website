mod contact;

use contact::ContactForm;
use yew::prelude::*;

struct ConstructionMessage {
    expected_end: String,
    title: String,
    message: String,
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let construction = ConstructionMessage {
            expected_end: "June 2022".to_string(),
            title: "Under Construction".to_string(),
            message: "Pardon our dust!".to_string(),
        };

        html! {
            <div align="center" class="main">
                <h1>{construction.title}</h1>
                <h3>{construction.message}</h3>
                <p>{ format!("Expected completion: {end}", end = construction.expected_end) }</p>
                <br/>
                <ContactForm/>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}

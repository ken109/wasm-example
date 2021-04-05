use yew::prelude::*;

pub struct Top {
    link: ComponentLink<Self>,
    value: f64,
}

pub enum Msg {
    Random,
}

impl Component for Top {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: js_sys::Math::random(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Random => self.value = js_sys::Math::random()
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::Random)>{ "ランダム" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

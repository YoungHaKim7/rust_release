use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

// yewfc     ~~~~~~~~~~~~~~
#[derive(PartialEq, Properties)]
pub struct ComponentNameProps {}

#[function_component]
pub fn ComponentName(props: &ComponentNameProps) -> Html {
    let ComponentNameProps {} = props;
    html! {
        <div></div>
    }
}
/////////////////////yewfc

// yewsc
pub struct ComponentName;

pub enum ComponentNameMsg {}

impl Component for ComponentName {
    type Message = ComponentNameMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {}
    }
}
////////////////////////yewsc

fn main() {
    yew::Renderer::<App>::new().render();
}

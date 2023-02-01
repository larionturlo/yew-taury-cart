use yew::prelude::*;


mod components;
use components::cart::CartComponent;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <CartComponent  />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
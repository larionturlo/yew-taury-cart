use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

mod tauri;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let name = use_state_eq(|| "".to_string());
    {
        let name = name.clone();
        use_effect_with_deps(move |_| {
            let name = name.clone();
            spawn_local(async move {
                name.set(tauri::get_name().await);
            });
        }, ());
    }

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
            <p>{ (*name).clone() }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
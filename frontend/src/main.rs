use product::{self, Product};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

mod tauri;
mod components;

use components::product_list::ProductsListComponent;

#[derive(Properties, PartialEq)]
struct ProductsListProps {
    products: Vec<Product>,
}

#[function_component]
fn App() -> Html {
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

    let error_msg = use_state_eq(|| "".to_string());
    let products = use_state(|| vec![]);
    {
        let products = products.clone();
        let error_msg = error_msg.clone();
        use_effect_with_deps(move |_| {
            let products = products.clone();
            let error_msg = error_msg.clone();
            spawn_local(async move {
                let result = tauri::get_products().await;
                match result {
                    Ok(prods) => products.set(prods), 
                    Err(error) => error_msg.set(error)
                }
            });
        }, ());
    }

    html! {
        <div>
            <p>{ (*name).clone() }</p>
            <p>{ (*error_msg).clone() }</p>

            <ProductsListComponent products={(*products).clone()} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
use std::collections::HashMap;

use yew::{html, Html, Context, Component, Callback};
use product::Product;

use crate::components::product_list::ProductsListComponent;
use yew_cart::tauri;

pub struct CartComponent {
    products: HashMap<String, Product>,
    error: Option<String>,
    state: Option<String>
}

impl CartComponent {
    fn modify_or_insert_product(&mut self, product: Product) {
        self.products
            .entry(product.name.clone())
            .and_modify(move |p| p.quantity = product.quantity )
            .or_insert(product.clone());
    }
}

pub enum Msg {
    Error(String),
    ProductChanged(Product),
    ProductAdded(Product),
    DownloadCart(),
}

impl Component for CartComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{
            products: HashMap::new(),
            error: None,
            state: Some("Loading...".to_owned())
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Error(err) => {
                self.error = Some(err);
                true
            },
            Msg::ProductChanged(p) => {
                self.modify_or_insert_product(p);
                true
            },
            Msg::ProductAdded(p) => {
                self.state = None;
                self.modify_or_insert_product(p);
                true
            },
            Msg::DownloadCart() => {
                let error_provider = ctx.link().callback(Msg::Error);
                let product_provider = ctx.link().callback(Msg::ProductAdded);
                download_products(product_provider, error_provider);
                false
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Msg::DownloadCart());
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let changes_provider = ctx.link().callback(Msg::ProductChanged);
        let err = if let Some(err) = &self.error {
            html!{<p>{ format!("Error: {err} ")}</p>}
        } else {
            html!{}
        };
        let state = if let Some(state) = &self.state {
            html!{<p>{ format!("State: {state} ")}</p>}
        } else {
            html!{}
        };
        html!{
            <div>
                <h1> { "Cart!" } </h1>
                {state}
                {err}
                <ProductsListComponent products={ self.products.clone() } on_edited={changes_provider}/>
            </div>
        }
    }
}

fn download_products(product_provider: Callback<Product>, error_provider: Callback<String>) {
    wasm_bindgen_futures::spawn_local(async move {
        let result = tauri::get_products().await;
        match result {
            Ok(prods) => {
                for p in &prods {
                    product_provider.emit((*p).to_owned())
                }
                ()
            },
            Err(error) => {
                error_provider.emit(error.into());
                ()
            }
        };
    })
}
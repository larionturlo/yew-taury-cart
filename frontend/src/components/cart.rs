use std::collections::HashMap;

use yew::{html, Html, Context, Component };
use product::Product;

use crate::components::product_list::ProductsListComponent;
use yew_cart::tauri;

pub struct CartComponent {
    products: HashMap<String, Product>,
    error: String,
    success: String,
    state: String
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
    ProductAdded(Product)
}

impl Component for CartComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link();
        async move {
            let result = tauri::get_products().await;
            link.send_message_batch(
                match result {
                    Ok(prods) => prods.iter()
                        .map(|p| Msg::ProductAdded((*p).to_owned()))
                        .collect::<Vec<_>>(),
                    Err(error) => vec![Msg::Error(error)]
                }
            );
        };

        Self{
            products: HashMap::new(),
            error: "".to_owned(),
            success: "".to_owned(),
            state: "Loading...".to_owned()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Error(err) => {
                self.error = err;
                true
            },
            Msg::ProductChanged(p) => {
                self.modify_or_insert_product(p);
                true
            },
            Msg::ProductAdded(p) => {
                self.modify_or_insert_product(p);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <div>
                <h1> { "Cart!" } </h1>
                <ProductsListComponent products={ self.products.clone() } />
            </div>
        }
    }
}
use std::collections::HashMap;

use product::Product;
use yew::{Properties, Component, Context, html, Html};

#[path = "product.rs"]
mod product_component;
use self::product_component::ProductComponent;



pub enum Msg {
    Recalc(),
    // QuantityProductChanged(String, i32)
    // DeleteProduct(String),
}

#[derive(Properties, PartialEq)]
pub struct ProductsListProps {
    pub products: Vec<Product>,
}


pub struct ProductsListComponent {
    products: HashMap<String, Product>,
    total_price: i32
}

impl ProductsListComponent {
    fn recalc_total_price(&mut self) {
        self.total_price = self.products.iter()
            .map(|p| p.1.price * p.1.quantity)
            .collect::<Vec<i32>>()
            .iter()
            .sum()
    }
}


impl Component for ProductsListComponent {
    type Message = Msg;
    type Properties = ProductsListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        //let total_price = 0;
        let products = _ctx.props().products.iter()
            .map(|product| (product.name.to_owned(), (*product).to_owned()) )
            .collect::<HashMap<String, Product>>();

        return Self{
            total_price: 0,
            products: products.clone()
        };
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Recalc() => {
                self.recalc_total_price();
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.products = _ctx.props().products.iter()
            .map(|product| (product.name.to_owned(), (*product).to_owned()) )
            .collect::<HashMap<String, Product>>();

        self.recalc_total_price();

        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let iner_html: Vec<Html> = self.products.iter().map(|product| html! {  
            <ProductComponent product={ (*product.1).clone() } />
        }).collect();

        html!{
            <div>
                <h2>{ "Cart" }</h2>
                <div>
                { iner_html }
                </div>
                <div>
                { format!{ "Total coast: ${:.2}", (self.total_price as f32)/100.0 } }
                </div>
            </div>
        }
    }
}
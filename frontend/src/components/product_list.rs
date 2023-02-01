use std::collections::HashMap;

use product::Product;
use yew::{Properties, Component, Context, html, Html};

use crate::components::product::ProductComponent;



pub enum Msg {
    Recalc(),
    // QuantityProductChanged(String, i32)
    // DeleteProduct(String),
}

#[derive(Properties, PartialEq)]
pub struct ProductsListProps {
    pub products: HashMap<String, Product>,
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

    fn create(ctx: &Context<Self>) -> Self {
        Self{
            total_price: 0,
            products: ctx.props().products.clone()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Recalc() => {
                self.recalc_total_price();
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.products = ctx.props().products.clone();
        self.recalc_total_price();

        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let iner_html: Vec<Html> = self.products.iter().map(|product| html! {  
            <ProductComponent product={ (*product.1).clone() } />
        }).collect();

        html!{
            <div>
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

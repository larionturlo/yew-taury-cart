use std::collections::HashMap;

use product::Product;
use yew::{Properties, Component, Context, html, Html, Callback};

use crate::components::product::ProductComponent;


#[derive(Properties, PartialEq)]
pub struct ProductsListProps {
    pub products: HashMap<String, Product>,
    pub on_edited: Callback<Product>
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
    type Message = ();
    type Properties = ProductsListProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self{
            total_price: 0,
            products: ctx.props().products.clone()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.products = ctx.props().products.clone();
        self.recalc_total_price();

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        
        let iner_html: Vec<Html> = self.products.iter().map(move |product| {
            let on_clicked = &ctx.props().on_edited;
            html! { <ProductComponent product={ (*product.1).clone() } {on_clicked}/>}
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

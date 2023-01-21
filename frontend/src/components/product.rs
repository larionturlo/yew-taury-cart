use product::Product;
use wasm_bindgen::JsValue;
use yew::{Properties, html, Html, Context, Component, Callback};

pub enum Msg {
    QuantityProductChanged(i32)
}
#[derive(Properties, PartialEq)]
pub struct ProductProps {
    pub product: Product,
}

pub struct ProductComponent {
    quantity: i32
}

impl Component for ProductComponent {
    type Message = Msg;
    type Properties = ProductProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let product = &_ctx.props().product;
        Self{
            quantity: product.quantity
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::QuantityProductChanged(q) => {
                self.quantity = q;

                web_sys::console::log_1(&JsValue::from_str(&*format!{"{:}", self.quantity}));
                true
            }
        }
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let product = &ctx.props().product;
        let increment_quantity = |q: i32| {
            move |_| { 
                Msg::QuantityProductChanged(q+1);
                ();
            }
        };
        let decrement_quantity = | q: i32| {
            move |_| {
                let deq = q-1;
                if deq < 0 {
                    return ;
                } 
                Msg::QuantityProductChanged(deq);
                ();
            }
        };
        html! {
            <div style={ "width: 33%; float: left" }>
                <div style={ "width: 100%" }>{ &product.name }</div>
                <div style={ "width: 50%; float:left;" }>
                    <button onclick={decrement_quantity(self.quantity)}>{ "-" }</button>
                    <input value={ format!{"{:}", self.quantity} } type="text" />
                    <button onclick={increment_quantity(self.quantity)}>{ "+" }</button>
                </div>
                <div style={ "width: 50%; float:right;" }>{ format!{"${:.2}",(product.price as f32)/100.0} }</div>
            </div>
        }
    }
}
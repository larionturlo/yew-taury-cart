use product::Product;
use wasm_bindgen::JsValue;
use yew::{Properties, html, Html, Context, Component, Callback};

pub enum Msg {
    QuantityProductIncrement(),
    QuantityProductDecrement()
}
#[derive(Properties, PartialEq)]
pub struct ProductProps {
    pub product: Product,
    pub on_clicked: Callback<Product>
}

pub struct ProductComponent {
    quantity: i32
}

impl Component for ProductComponent {
    type Message = Msg;
    type Properties = ProductProps;

    fn create(ctx: &Context<Self>) -> Self {
        let product = &ctx.props().product;
        Self{
            quantity: product.quantity
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::QuantityProductIncrement() => {
                self.quantity += 1;
                let mut p = ctx.props().product.clone();
                p.quantity = self.quantity;
                ctx.props().on_clicked.emit(p);

                web_sys::console::log_1(&JsValue::from_str(&*format!{"{:}", self.quantity}));
                true
            }
            Msg::QuantityProductDecrement() => {
                if self.quantity > 0 {
                    self.quantity -= 1;
                    let mut p = ctx.props().product.clone();
                    p.quantity = self.quantity;
                    ctx.props().on_clicked.emit(p);
                    web_sys::console::log_1(&JsValue::from_str(&*format!{"{:}", self.quantity}));
                    true
                } else {
                    false
                }
            }
        }
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let product = &ctx.props().product;
        let increment_quantity = ctx.link().callback(|_| Msg::QuantityProductIncrement());
        let decrement_quantity = ctx.link().callback(|_| Msg::QuantityProductDecrement()); 
        html! {
            <div style={ "width: 33%; float: left" }>
                <div style={ "width: 100%;" }>{ &product.name }</div>
                <div style={ "width: 50%; float:left;" }>
                    <button onclick={decrement_quantity}>{ "-" }</button>
                    <input value={ format!{"{:}", self.quantity} } type="text" />
                    <button onclick={increment_quantity}>{ "+" }</button>
                </div>
                <div style={ "width: 50%; float:right;" }>{ format!{"${:.2}",(product.price as f32)/100.0} }</div>
            </div>
        }
    }
}
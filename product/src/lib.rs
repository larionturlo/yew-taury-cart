use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Product {
    pub name: String,
    pub price: f32,
    pub quantity: i32,
}

pub type ProductList = Vec<Product>;

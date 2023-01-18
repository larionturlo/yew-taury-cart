use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Product {
    pub name: String,
    pub price: f32,
    pub quantity: i32,
}

pub type ProductList = Vec<Product>;

pub fn parse_products_from_json(json: String) -> Result<ProductList, String> {
    let prods = serde_json::from_str::<ProductList>(&json);

    match prods {
        Ok(products) => Ok(products),
        Err(message) => Err(message.to_string())
    }
}

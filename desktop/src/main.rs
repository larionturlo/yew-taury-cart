#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use product::ProductList;
use tauri::{generate_context, generate_handler};

pub fn parse_products_from_json(json: String) -> Result<ProductList, String> {
  let prods = serde_json::from_str::<ProductList>(&json);

  match prods {
      Ok(products) => Ok(products),
      Err(message) => Err(message.to_string())
  }
}

#[tauri::command]
fn get_products() -> Result<product::ProductList, String> {
  let products_json = std::fs::read_to_string("./products.json");

  match products_json {
      Ok(products_json_string) => parse_products_from_json(products_json_string),
      Err(msg) => Err(msg.to_string())
  } 
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(generate_handler![get_products])
    .run(generate_context!())
    .expect("error while running tauri application");
}

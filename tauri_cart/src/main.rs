#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use product;
use tauri::{generate_context, generate_handler};

#[tauri::command]
fn get_products() -> Result<product::ProductList, String> {
  let products_json = std::fs::read_to_string("./products.json");

  match products_json {
      Ok(products_json_string) => product::parse_products_from_json(products_json_string),
      Err(msg) => Err(msg.to_string())
  } 
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(generate_handler![get_products])
    .run(generate_context!())
    .expect("error while running tauri application");
}

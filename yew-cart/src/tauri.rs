use std::ops::Add;

use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "app"])]
    fn getName() -> Promise;
}

pub async fn get_name() -> String {
    return match JsFuture::from(getName()).await {
        Ok(name) => match name.as_string() {
            Some(name) => name.add(" test"),
            None => "noname".to_owned()
        },
        Err(_) => "noname".to_owned()
    };
}
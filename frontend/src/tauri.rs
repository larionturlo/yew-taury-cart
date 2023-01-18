use js_sys::Promise;
use product::ProductList;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::prelude::*;
use gloo_utils::format::JsValueSerdeExt;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "app"])]
    fn getName() -> Promise;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__"])]
    fn invoke(command: JsValue, params: JsValue) -> Promise;
}

pub async fn get_name() -> String {
    return match JsFuture::from(getName()).await {
        Ok(name) => match name.as_string() {
            Some(name) => name,
            None => "noname".to_owned()
        },
        Err(_) => "noname".to_owned()
    };
}

pub async fn get_products() -> Result<ProductList,String> {
    return match JsFuture::from(invoke(JsValue::from_str("get_products"), JsValue::null())).await {
        Ok(data) => match data.into_serde::<ProductList>() {
            Ok(products) => Ok(products),
            Err(message) => Err(message.to_string())
        },
        Err(msg) =>  Err(match msg.as_string() {
            Some(msg) => msg,
            None => "Unknown error".to_owned()
        }),
    };
}
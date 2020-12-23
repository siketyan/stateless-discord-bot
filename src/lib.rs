mod context;
mod discord;
mod error;
mod http;
mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

use crate::context::Context;
use crate::http::HttpResponse;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn wasm_main(context: &JsValue) -> JsValue {
    JsValue::from_serde(
        &(match context.into_serde::<Context>() {
            Ok(ctx) => ctx.handle_http_request(),
            Err(error) => HttpResponse {
                status: 400,
                body: error.to_string(),
            },
        }),
    )
    .unwrap()
}

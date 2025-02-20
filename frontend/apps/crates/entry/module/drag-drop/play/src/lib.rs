#![feature(type_alias_impl_trait)]

//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod router;
mod debug;
mod strings;
mod state;
mod base;
mod config;

use wasm_bindgen::prelude::*;
use std::rc::Rc;
use router::Router;

#[wasm_bindgen(start)]
pub async fn main_js() {
    utils::panic_hook::set_hook();
    utils::logging::setup_logging();

    utils::init::init().await;

    let router = Rc::new(Router::new());

    router::render(router.clone());

    //std::mem::forget(Box::new(router));
}

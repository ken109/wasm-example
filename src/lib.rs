extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use std::panic;

mod util;
mod pages;

#[wasm_bindgen(start)]
pub fn run_app() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    App::<pages::top::Top>::new().mount_to_body();
}

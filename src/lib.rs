#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

extern crate console_error_panic_hook;
use std::panic;

#[wasm_bindgen]
pub fn my_init_function() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    alert(&format!("Hello, wasm-cetkaik! {}", s));
}

use cetkaik_random_play::yield_random_next;

#[wasm_bindgen]
pub fn send_example_to_js() -> JsValue {
    let initial_state = cetkaik_full_state_transition::initial_state();
    let initial_state: cetkaik_full_state_transition::probabilistic::Prob<_> = initial_state.into();
    let (initial_state, _nonexistent_ciurl) = initial_state.choose();

    let mut game_buffer = vec![initial_state.clone()];

    if let Some(next_state) = yield_random_next(&initial_state, cetkaik_full_state_transition::Config::cerke_online_alpha()) {
        game_buffer.push(next_state);
    }

    JsValue::from_serde(&game_buffer).unwrap()
}

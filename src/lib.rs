#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

use once_cell::sync::Lazy;
static GAME_BUFFER: Lazy<Vec<cetkaik_full_state_transition::state::A>> = Lazy::new(|| {
    use cetkaik_full_state_transition::*;
    let mut ans = vec![];

    let initial_state = initial_state();
    let initial_state: probabilistic::Prob<_> = initial_state.into();
    let (initial_state, _nonexistent_ciurl) = initial_state.choose();

    let mut current_state: state::A = initial_state;

    loop {
        use cetkaik_full_state_transition::message::binary::Binary;
        let (adequate_normal_move, resulting_prob_density) = loop {
            let msg = message::NormalMove::random_choice();
            match apply_normal_move(&current_state, msg, Config::cerke_online_alpha()) {
                Err(_) => continue,
                Ok(prob) => break (msg, prob),
            }
        };

        let resulting_prob_density: probabilistic::Prob<_> = resulting_prob_density.into();
        let (hand_not_resolved, maybe_ciurl) = resulting_prob_density.choose();
        let next_state = match resolve(&hand_not_resolved, Config::cerke_online_alpha()) {
            state::HandResolved::HandExists { if_tymok, if_taxot } => {
                // FIXME: always chooses taxot
                match if_taxot {
                    IfTaxot::VictoriousSide(victor) => break,
                    IfTaxot::NextSeason(prob_distribution) => {
                        let prob_distribution: probabilistic::Prob<_> = prob_distribution.into();
                        let (next_state, _nonexistent_ciurl) = prob_distribution.choose();
                        next_state
                    }
                }
            }
            state::HandResolved::NeitherTymokNorTaxot(next_state) => next_state,
            state::HandResolved::GameEndsWithoutTymokTaxot(_) => break,
        };
        ans.push(next_state.clone());
        current_state = next_state;
        
        if ans.len() >= 20 {
            break;
        } else {
            continue;
        }
    }

    ans
});

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

#[wasm_bindgen]
pub fn get_game_length() -> usize {
    GAME_BUFFER.len()
}

#[wasm_bindgen]
pub fn send_example_to_js(ind: usize) -> JsValue {
    JsValue::from_serde(&GAME_BUFFER[ind]).unwrap()
}

#[wasm_bindgen]
pub fn dump_game_buffer() -> JsValue {
    JsValue::from_serde(&*GAME_BUFFER).unwrap()
}

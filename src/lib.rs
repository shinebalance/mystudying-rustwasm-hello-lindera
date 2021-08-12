extern crate serde_json;
use lindera::tokenizer::Tokenizer;
use lindera_core::core::viterbi::Mode;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn greet(text: &str) -> JsValue {

    // create tokenizer
    let mut tokenizer = Tokenizer::new(Mode::Normal, "");

    // tokenize the text
    let tokens = tokenizer.tokenize(&text);

    // JSに対応した戻り値を返すので
    JsValue::from_serde(&tokens).unwrap()

}

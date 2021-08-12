use lindera::tokenizer::Tokenizer;
use lindera_core::core::viterbi::Mode;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(text: &str) {

    // create tokenizer
    let mut tokenizer = Tokenizer::new(Mode::Normal, "");

    // tokenize the text
    let tokens = tokenizer.tokenize(&text);

    // output the tokens
    for token in tokens {
        alert(&format!("-->{}", token.text));
    }

}

// fn main() -> std::io::Result<()> {
//     // create tokenizer
//     let mut tokenizer = Tokenizer::new(Mode::Normal, "");

//     // tokenize the text
//     let tokens = tokenizer.tokenize("東京都の新型コロナウイルスのモニタリング会議が１２日あり、都内の感染状況について専門家から「制御不能な状況だ。災害レベルで感染が猛威を振るう非常事態」と報告された。");

//     // output the tokens
//     for token in tokens {
//         println!("{}", token.text);
//     }

//     Ok(())
// }
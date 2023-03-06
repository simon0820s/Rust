use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern{
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn say_hi(name: &str){
    alert(&format!("Hello, {} how are you?",name));
}
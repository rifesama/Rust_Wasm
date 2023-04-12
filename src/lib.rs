use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert (S: &str);
    
}

#[wasm_bindgen]
pub fn saludar(nombre: &str) {
    alert(&format!("Hola {}, ¿Como estas?", nombre))
}
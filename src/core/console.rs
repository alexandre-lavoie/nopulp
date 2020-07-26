use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Binds to the external javascript console.log.
    ///
    /// TODO: Should implement macro.
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(a: &str);

    /// Binds to the external javascript console.error.
    ///
    /// TODO: Should implement macro.
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(a: &str);
}

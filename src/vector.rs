use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen()]
impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Default for Vector {
    fn default() -> Self {
        Self { x: 0., y: 0. }
    }
}

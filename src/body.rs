use crate::vector::Vector;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen()]
#[derive(Serialize, Deserialize, Debug)]
pub struct JsBody {
    pub id: usize,
    pub position: Vector,
}

#[wasm_bindgen()]
pub struct Body {
    pub position: Vector,
    pub last_position: Vector,
    mass: f64,
    forces: Vector,
}

impl Body {
    pub fn new(position: Vector) -> Self {
        Self {
            position,
            last_position: position,
            forces: Vector::default(),
            mass: 1.,
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        let y_acell = (self.forces.y / self.mass) * 240.;
        let x_acell = (self.forces.x / self.mass) * 240.;
        let y_travel = self.position.y - self.last_position.y;
        let x_travel = self.position.x - self.last_position.x;
        let squared_delta_time = delta_time * delta_time;

        self.last_position = self.position;
        self.forces = Vector::default();

        self.position = Vector {
            x: self.position.x + x_travel + x_acell * squared_delta_time,
            y: self.position.y + y_travel + y_acell * squared_delta_time,
        }
    }

    pub fn add_force(&mut self, vector: Vector) {
        self.forces.x += vector.x;
        self.forces.y += vector.y;
    }
}

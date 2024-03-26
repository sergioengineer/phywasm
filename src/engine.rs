use crate::body::{Body, JsBody};
use crate::vector::Vector;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Array;
use web_sys::{console, window};

#[wasm_bindgen()]
extern "C" {
    fn alert(msg: &str);
}

static GRAVITY: Vector = Vector { x: 0., y: -10. };
#[wasm_bindgen()]
pub struct Engine {
    bodies: Vec<Option<Body>>,
    last_execution_time: Option<f64>,
}

#[wasm_bindgen()]
impl Engine {
    pub fn new(max_bodies: usize) -> Engine {
        let mut eng = Engine {
            bodies: Vec::with_capacity(max_bodies),
            last_execution_time: None,
        };

        for _n in 0..max_bodies {
            eng.bodies.push(None);
        }
        eng
    }

    pub fn add_body(&mut self, position: Vector) -> Option<usize> {
        let free_slot_index = self.bodies.iter().position(|x| x.is_none());

        match free_slot_index {
            Some(free_slot_index) => {
                console::log(&Array::from(&JsValue::from_str(
                    &("added: ".to_owned() + &free_slot_index.to_string()),
                )));
                self.bodies
                    .insert(free_slot_index, Some(Body::new(position)));
                Some(free_slot_index)
            }
            None => {
                console::log(&Array::from(&JsValue::from_str("não tem free")));
                None
            }
        }
    }

    pub fn remove_body(&mut self, id: usize) {
        if self.bodies.len() - 1 < id {
            return;
        }

        self.bodies[id] = None;
    }

    pub fn update(&mut self, now: f64) {
        if self.last_execution_time.is_none() {
            self.last_execution_time = Some(now);
            return;
        }

        let delta_time = (now - self.last_execution_time.unwrap()) * 0.001;
        self.last_execution_time = Some(now);
        let mut body_events: Vec<JsBody> = Vec::with_capacity(self.bodies.len());

        for n in 0..self.bodies.len() {
            if let Some(body) = self.bodies[n].as_mut() {
                body.add_force(GRAVITY);
                body.update(delta_time);

                let body_event = JsBody {
                    id: n,
                    position: body.position,
                };
                body_events.push(body_event);
            }
        }

        let _ = window().unwrap().post_message(
            &JsValue::from_str(&serde_json::to_string(&body_events).unwrap()),
            "*",
        );
    }
}

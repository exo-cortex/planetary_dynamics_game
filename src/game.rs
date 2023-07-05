use serde::{Deserialize, Serialize};

pub type time = f64;

#[derive(Serialize, Deserialize, Debug)]
pub struct MassiveObject {
    location: [f64; 2],
    velocity: [f64; 2],
    mass: f64,
}

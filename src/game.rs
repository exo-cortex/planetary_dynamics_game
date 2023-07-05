use serde::{Deserialize, Serialize};

pub type Time = f64;
pub type PlayerId = u8;

#[derive(Serialize, Deserialize, Debug)]
pub struct MassiveObject {
    pub location: [f64; 2],
    pub velocity: [f64; 2],
    pub mass: f64,
    pub color: Option<[u8; 3]>,
}

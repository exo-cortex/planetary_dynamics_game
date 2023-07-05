use serde::{Deserialize, Serialize};

use crate::game::{MassiveObject, PlayerId, Time};

#[derive(Serialize, Deserialize, Debug)]
pub enum PlayerAction {
    NewObject(Time, MassiveObject),
    PauseGame,
    UnPauseGame,
    WriteMessage(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameUpdate {
    pub player: PlayerId,
    pub action: PlayerAction,
}

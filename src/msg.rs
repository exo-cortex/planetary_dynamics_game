use serde::{Deserialize, Serialize};

use crate::game::{time, MassiveObject};

#[derive(Serialize, Deserialize, Debug)]
enum PlayerAction {
    NewObject(time, MassiveObject),
    PauseGame,
    UnPauseGame,
}

#[derive(Serialize, Deserialize, Debug)]
enum GameUpdate {
    PlayerAction(u8, PlayerAction),
}

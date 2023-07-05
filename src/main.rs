mod game;
mod msg;

use crate::{
    game::MassiveObject,
    msg::{GameUpdate, PlayerAction},
};

fn main() {
    let update: GameUpdate = GameUpdate {
        player: 1,
        action: PlayerAction::NewObject(
            10.0,
            MassiveObject {
                location: [0.25, 0.8],
                velocity: [-0.05, 0.01],
                mass: 3.21,
                color: None,
            },
        ),
    };

    let update2: GameUpdate = GameUpdate {
        player: 2,
        action: PlayerAction::WriteMessage("fuck you lololol!".to_string()),
    };

    let bytes: Vec<u8> = bincode::serialize(&update).unwrap();
    println!("number of bytes: {}\n data: {:02X?}", &bytes.len(), &bytes,);

    let newupdate: GameUpdate = bincode::deserialize(&bytes).unwrap();
    println!("bytes: {:?}", newupdate);

    match &newupdate.action {
        PlayerAction::NewObject(_at_time, _new_object) => {
            println!("player #{} created new massive object with coordinates [], velocity [] and mass []", newupdate.player);
        }
        PlayerAction::PauseGame => {
            println!("player #{} paused the game", newupdate.player);
        }
        PlayerAction::UnPauseGame => {
            println!("player #{} unpaused the game", newupdate.player);
        }
        PlayerAction::WriteMessage(msg) => {
            println!("player #{} wrote: \"{}\"", newupdate.player, msg);
        }
    }
}

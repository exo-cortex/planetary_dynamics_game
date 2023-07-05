use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct UpdateMsg {
    data: [u8; 3],
}

fn main() {
    let update: UpdateMsg = UpdateMsg { data: [1, 2, 3] };
    let bytes = bincode::serialize(&update).unwrap();
    println!("bytes: {:?}", bytes);

    let newupdate: UpdateMsg = bincode::deserialize(&bytes).unwrap();
    println!("bytes: {:?}", newupdate);
}

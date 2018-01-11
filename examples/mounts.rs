extern crate linmon;

use linmon::mounts::{Mount, Mounts};

fn main() {
    // get running PIDs of whole system
    let ms = Mounts::new();
    // fetch process state for each PID
    for m in ms.get_mounts() {
        println!("{}", m)
    }
}

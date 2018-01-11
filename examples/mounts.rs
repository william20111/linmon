extern crate linmon;

use linmon::mounts::{Mount, Mounts};

fn main() {
    // get system mounts
    let ms = Mounts::new();
    // fetch
    for m in ms.get_mounts() {
        println!("{}", m)
    }
}

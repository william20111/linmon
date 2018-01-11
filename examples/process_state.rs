extern crate linmon;

use linmon::process::{Process, Processes};

fn main() {
    // get running PIDs of whole system
    let ps = Processes::new();
    println!("{:?}", ps);
    // fetch process state for each PID
    for p in ps.processes() {
        let x = Process::new(&p);
        println!("{}", x.get_state());
    }
}

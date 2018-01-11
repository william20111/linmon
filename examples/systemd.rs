extern crate linmon;

fn main() {
    let ps = linmon::procfs::process::Processes::new();
    println!("{:?}", ps);
    for p in ps.processes() {
        let x = linmon::procfs::process::Process::new(p.to_string().as_str());
        println!("{}", x.get_pid());
        println!("{}", x.get_state());
    }
}

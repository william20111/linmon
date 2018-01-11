extern crate linmon;

fn main() {
    let ps = linmon::procfs::process::Processes::new();
    for p in ps.processes() {
        let x = linmon::procfs::process::Process::new(p.to_string().as_str());
        println!("{:?}", x);
    }
}

# Linmon

[![Build Status](https://travis-ci.org/william20111/linmon.svg?branch=master)](https://travis-ci.org/william20111/linmon)

Library for Linux system metrics and stats

[Documentation](https://docs.rs/linmon/0.1/linmon/)

## Overview

Currenly a WIP and considered very unstable and API will change. 

```rust
use linmon::procfs::Process;

fn main() {
    // get running PIDs of whole system
    let ps = linmon::procfs::process::Processes::new();
    println!("{:?}", ps);
    // fetch process state for each PID
    for p in ps.processes() {
        let x = linmon::procfs::process::Process::new(p.to_string().as_str());
        println!("{}", x.get_state());
    }
}
```

```rust
use linmon::mounts::{Mount, Mounts};

fn main() {
    // get system mounts
    let ms = Mounts::new();
    // print mounts
    for m in ms.get_mounts() {
        println!("{}", m)
    }
}
```

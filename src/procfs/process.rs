use std::fs;
use std::str;
use std::fmt;
use std::io::Read;
use std::io;
use regex::Regex;
use std::path::Path;
use std::str::FromStr;

pub static PROC: &str = "/proc";

#[derive(Debug)]
enum ProcessState {
    Running,
    Sleeping,
    Waiting,
    Zombie,
    Stopped,
    Tracing,
    Dead,
    Wakekill,
    Waking,
    Parked,
    Idle,
}

impl PartialEq for ProcessState {
    fn eq(&self, other: &ProcessState) -> bool {
        self == other
    }
}

impl ToString for ProcessState {
    fn to_string(&self) -> String {
        match *self {
            ProcessState::Running => "R".to_string(),
            ProcessState::Sleeping => "S".to_string(),
            ProcessState::Waiting => "D".to_string(),
            ProcessState::Zombie => "Z".to_string(),
            ProcessState::Stopped => "T".to_string(),
            ProcessState::Tracing => "t".to_string(),
            ProcessState::Dead => "X".to_string(),
            ProcessState::Wakekill => "K".to_string(),
            ProcessState::Waking => "W".to_string(),
            ProcessState::Parked => "P".to_string(),
            ProcessState::Idle => "I".to_string(),
        }
    }
}

impl FromStr for ProcessState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Running
            "R" => Ok(ProcessState::Running),
            // Sleeping in an interruptible wait
            "S" => Ok(ProcessState::Sleeping),
            // Waiting in uninterruptible disk sleep
            "D" => Ok(ProcessState::Waiting),
            // Zombie
            "Z" => Ok(ProcessState::Zombie),
            // Stopped (on a signal) or (before Linux 2.6.33) trace stopped
            "T" => Ok(ProcessState::Stopped),
            // Tracing stop (Linux 2.6.33 onward)
            "t" => Ok(ProcessState::Tracing),
            // Dead (from Linux 2.6.0 onward)
            "X" => Ok(ProcessState::Dead),
            // Wakekill (Linux 2.6.33 to 3.13 only)
            "K" => Ok(ProcessState::Wakekill),
            "W" => Ok(ProcessState::Waking),
            // Parked (Linux 3.9 to 3.13 only)
            "P" => Ok(ProcessState::Parked),
            // Idle
            "I" => Ok(ProcessState::Idle),
            _ => Err(format!("Failed to parse state {}", s)),
        }
    }
}

#[derive(Debug)]
pub struct Processes {
    processes: Vec<i64>,
}

impl Processes {
    pub fn processes(self) -> Vec<i64> {
        self.processes
    }

    pub fn new() -> Processes {
        Processes::fetch()
    }

    fn fetch() -> Processes {
        let mut procs = vec![];
        let re = Regex::new(r"^\d+$").unwrap();
        for path in fs::read_dir(PROC).unwrap() {
            let path = path.unwrap();
            match re.is_match(path.path().file_name().unwrap().to_str().unwrap()) {
                true => procs.push(
                    path.path()
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                ),
                false => {}
            }
        }
        Processes { processes: procs }
    }
}

#[derive(Debug)]
pub struct Process {
    pid: i64,
    comm: String,
    state: ProcessState,
    ppid: i64,
    pgrp: i64,
    session: i64,
    tty_nr: i64,
    tpgid: i64,
    flags: i64,
    minflt: i64,
    cminflt: i64,
    majflt: i64,
    cmajflt: i64,
    utime: i64,
    stime: i64,
    cutime: i64,
    cstime: i64,
    priority: i64,
    nice: i64,
    num_threads: i64,
    itrealvalue: i64,
    starttime: i128,
    vsize: i64,
    rss: i64,
    rsslim: i128,
    startcode: i64,
    endcode: i64,
    startstack: i64,
    kstkesp: i64,
    signal: i64,
    blocked: i64,
    sigignore: i64,
    sigcatch: i64,
    wchan: i64,
    nswap: i64,
    cnswap: i64,
    exit_signal: i64,
    processor: i64,
    rt_priority: i64,
    policy: i64,
    delayacct_blkio_ticks: i128,
    guest_time: i64,
    cguest_time: i64,
    start_data: i64,
    end_data: i64,
    start_brk: i64,
    arg_start: i64,
    arg_end: i64,
    env_start: i64,
    env_end: i64,
    exit_code: i64,
}

impl Process {
    pub fn is_alive(p: i64) -> bool {
        let pid = format!("/proc/{}", p);
        Path::new(pid.as_str()).exists()
    }

    pub fn get_pid(&self) -> i64 {
        self.pid
    }

    pub fn get_state(&self) -> String {
        self.state.to_string()
    }

    pub fn cmdline(p: i64) -> String {
        let path = format!("/proc/{}/cmdline", p);
        match fs::File::open(path) {
            Ok(f) => {
                let mut s = String::new();
                let mut buf = io::BufReader::new(f);
                buf.read_to_string(&mut s).unwrap();
                s
            }
            Err(_) => "Failed to find PID".to_string(),
        }
    }

    pub fn new(p: &str) -> Process {
        let s = Process::fetch(p).unwrap();
        Process::parse(s)
    }

    fn fetch(pid: &str) -> Result<String, io::Error> {
        let path = format!("/{}/{}/stat", PROC, pid);
        let mut s = String::new();
        match fs::File::open(path) {
            Ok(f) => {
                let mut buf = io::BufReader::new(f);
                buf.read_to_string(&mut s).unwrap();
                Ok(s)
            }
            Err(e) => Err(e),
        }
    }

    fn parse(s: String) -> Process {
        let store: Vec<&str> = s.split_whitespace().collect();
        if store[1].starts_with("(") && store[2].ends_with(")") {
            Process {
                pid: store[0].parse::<i64>().unwrap(),
                comm: format!("{} {}", store[1], store[2]),
                state: ProcessState::from_str(store[3]).unwrap(),
                ppid: store[4].parse::<i64>().unwrap(),
                pgrp: store[5].parse::<i64>().unwrap(),
                session: store[6].parse::<i64>().unwrap(),
                tty_nr: store[7].parse::<i64>().unwrap(),
                tpgid: store[8].parse::<i64>().unwrap(),
                flags: store[9].parse::<i64>().unwrap(),
                minflt: store[10].parse::<i64>().unwrap(),
                cminflt: store[11].parse::<i64>().unwrap(),
                majflt: store[12].parse::<i64>().unwrap(),
                cmajflt: store[13].parse::<i64>().unwrap(),
                utime: store[14].parse::<i64>().unwrap(),
                stime: store[15].parse::<i64>().unwrap(),
                cutime: store[16].parse::<i64>().unwrap(),
                cstime: store[17].parse::<i64>().unwrap(),
                priority: store[18].parse::<i64>().unwrap(),
                nice: store[19].parse::<i64>().unwrap(),
                num_threads: store[20].parse::<i64>().unwrap(),
                itrealvalue: store[21].parse::<i64>().unwrap(),
                starttime: store[22].parse::<i128>().unwrap(),
                vsize: store[23].parse::<i64>().unwrap(),
                rss: store[24].parse::<i64>().unwrap(),
                rsslim: store[25].parse::<i128>().unwrap(),
                startcode: store[26].parse::<i64>().unwrap(),
                endcode: store[27].parse::<i64>().unwrap(),
                startstack: store[28].parse::<i64>().unwrap(),
                kstkesp: store[29].parse::<i64>().unwrap(),
                signal: store[30].parse::<i64>().unwrap(),
                blocked: store[31].parse::<i64>().unwrap(),
                sigignore: store[32].parse::<i64>().unwrap(),
                sigcatch: store[33].parse::<i64>().unwrap(),
                wchan: store[34].parse::<i64>().unwrap(),
                nswap: store[35].parse::<i64>().unwrap(),
                cnswap: store[36].parse::<i64>().unwrap(),
                exit_signal: store[37].parse::<i64>().unwrap(),
                processor: store[38].parse::<i64>().unwrap(),
                rt_priority: store[39].parse::<i64>().unwrap(),
                policy: store[40].parse::<i64>().unwrap(),
                delayacct_blkio_ticks: store[41].parse::<i128>().unwrap(),
                guest_time: store[42].parse::<i64>().unwrap(),
                cguest_time: store[43].parse::<i64>().unwrap(),
                start_data: store[44].parse::<i64>().unwrap(),
                end_data: store[45].parse::<i64>().unwrap(),
                start_brk: store[46].parse::<i64>().unwrap(),
                arg_start: store[47].parse::<i64>().unwrap(),
                arg_end: store[48].parse::<i64>().unwrap(),
                env_start: store[49].parse::<i64>().unwrap(),
                env_end: store[50].parse::<i64>().unwrap(),
                exit_code: store[51].parse::<i64>().unwrap(),
            }
        } else {
            Process {
                pid: store[0].parse::<i64>().unwrap(),
                comm: store[1].to_string(),
                state: ProcessState::from_str(store[2]).unwrap(),
                ppid: store[3].parse::<i64>().unwrap(),
                pgrp: store[4].parse::<i64>().unwrap(),
                session: store[5].parse::<i64>().unwrap(),
                tty_nr: store[6].parse::<i64>().unwrap(),
                tpgid: store[7].parse::<i64>().unwrap(),
                flags: store[8].parse::<i64>().unwrap(),
                minflt: store[9].parse::<i64>().unwrap(),
                cminflt: store[10].parse::<i64>().unwrap(),
                majflt: store[11].parse::<i64>().unwrap(),
                cmajflt: store[12].parse::<i64>().unwrap(),
                utime: store[13].parse::<i64>().unwrap(),
                stime: store[14].parse::<i64>().unwrap(),
                cutime: store[15].parse::<i64>().unwrap(),
                cstime: store[16].parse::<i64>().unwrap(),
                priority: store[17].parse::<i64>().unwrap(),
                nice: store[18].parse::<i64>().unwrap(),
                num_threads: store[19].parse::<i64>().unwrap(),
                itrealvalue: store[20].parse::<i64>().unwrap(),
                starttime: store[21].parse::<i128>().unwrap(),
                vsize: store[22].parse::<i64>().unwrap(),
                rss: store[23].parse::<i64>().unwrap(),
                rsslim: store[24].parse::<i128>().unwrap(),
                startcode: store[25].parse::<i64>().unwrap(),
                endcode: store[26].parse::<i64>().unwrap(),
                startstack: store[27].parse::<i64>().unwrap(),
                kstkesp: store[28].parse::<i64>().unwrap(),
                signal: store[29].parse::<i64>().unwrap(),
                blocked: store[30].parse::<i64>().unwrap(),
                sigignore: store[31].parse::<i64>().unwrap(),
                sigcatch: store[32].parse::<i64>().unwrap(),
                wchan: store[33].parse::<i64>().unwrap(),
                nswap: store[34].parse::<i64>().unwrap(),
                cnswap: store[35].parse::<i64>().unwrap(),
                exit_signal: store[36].parse::<i64>().unwrap(),
                processor: store[37].parse::<i64>().unwrap(),
                rt_priority: store[38].parse::<i64>().unwrap(),
                policy: store[39].parse::<i64>().unwrap(),
                delayacct_blkio_ticks: store[40].parse::<i128>().unwrap(),
                guest_time: store[41].parse::<i64>().unwrap(),
                cguest_time: store[42].parse::<i64>().unwrap(),
                start_data: store[43].parse::<i64>().unwrap(),
                end_data: store[44].parse::<i64>().unwrap(),
                start_brk: store[44].parse::<i64>().unwrap(),
                arg_start: store[45].parse::<i64>().unwrap(),
                arg_end: store[46].parse::<i64>().unwrap(),
                env_start: store[47].parse::<i64>().unwrap(),
                env_end: store[48].parse::<i64>().unwrap(),
                exit_code: store[49].parse::<i64>().unwrap(),
            }
        }
    }
}

impl PartialEq for Process {
    fn eq(&self, other: &Process) -> bool {
        (self.comm == other.comm) & (self.state == other.state) & (self.ppid == other.ppid)
            & (self.pgrp == other.pgrp) & (self.session == other.session)
            & (self.tty_nr == other.tty_nr) & (self.tpgid == other.tpgid)
            & (self.flags == other.flags) & (self.minflt == other.minflt)
            & (self.cminflt == other.cminflt) & (self.majflt == other.majflt)
            & (self.cmajflt == other.cmajflt) & (self.utime == other.utime)
            & (self.stime == other.stime) & (self.cutime == other.cutime)
            & (self.cstime == other.cstime) & (self.priority == other.priority)
            & (self.nice == other.nice) & (self.num_threads == other.num_threads)
            & (self.itrealvalue == other.itrealvalue) & (self.starttime == other.starttime)
            & (self.vsize == other.vsize) & (self.rss == other.rss)
            & (self.rsslim == other.rsslim) & (self.startcode == other.startcode)
            & (self.endcode == other.endcode) & (self.startstack == other.startstack)
            & (self.kstkesp == other.kstkesp) & (self.signal == other.signal)
            & (self.blocked == other.blocked) & (self.sigignore == other.sigignore)
            & (self.sigcatch == other.sigcatch) & (self.wchan == other.wchan)
            & (self.nswap == other.nswap) & (self.cnswap == other.cnswap)
            & (self.exit_signal == other.exit_signal) & (self.processor == other.processor)
            & (self.rt_priority == other.rt_priority) & (self.policy == other.policy)
            & (self.delayacct_blkio_ticks == other.delayacct_blkio_ticks)
            & (self.guest_time == other.guest_time)
            & (self.cguest_time == other.cguest_time)
            & (self.start_data == other.start_data) & (self.end_data == other.end_data)
            & (self.start_brk == other.start_brk) & (self.arg_start == other.arg_start)
            & (self.arg_end == other.arg_end) & (self.env_start == other.env_start)
            & (self.env_end == other.env_end) & (self.exit_code == other.exit_code)
    }
}

impl fmt::Display for Process {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "Process: {} {}", self.comm, self.ppid)
    }
}

#[test]
fn test_process_parse_412_kernel() {
    let pid1 = "1 (tmux: client) S 0 1 1 0 -1 4210944 61449 62034795 78 5512 161 316 380282 71962 20 0 1 0 5 229601280 3065 18446744073709551615 1 1 0 0 0 0 671173123 4096 1260 0 0 0 17 0 0 0 65755103 0 0 0 0 0 0 0 0 0 0";
    let p = Process::parse(pid1.to_string());
    let test = Process {
        pid: "1".parse::<i64>().unwrap(),
        comm: "(tmux: client)".to_string(),
        state: ProcessState::Sleeping,
        ppid: "0".parse::<i64>().unwrap(),
        pgrp: "1".parse::<i64>().unwrap(),
        session: "1".parse::<i64>().unwrap(),
        tty_nr: "0".parse::<i64>().unwrap(),
        tpgid: "-1".parse::<i64>().unwrap(),
        flags: "4210944".parse::<i64>().unwrap(),
        minflt: "61449".parse::<i64>().unwrap(),
        cminflt: "62034795".parse::<i64>().unwrap(),
        majflt: "78".parse::<i64>().unwrap(),
        cmajflt: "5512".parse::<i64>().unwrap(),
        utime: "161".parse::<i64>().unwrap(),
        stime: "316".parse::<i64>().unwrap(),
        cutime: "380282".parse::<i64>().unwrap(),
        cstime: "71962".parse::<i64>().unwrap(),
        priority: "20".parse::<i64>().unwrap(),
        nice: "0".parse::<i64>().unwrap(),
        num_threads: "1".parse::<i64>().unwrap(),
        itrealvalue: "0".parse::<i64>().unwrap(),
        starttime: "5".parse::<i128>().unwrap(),
        vsize: "229601280".parse::<i64>().unwrap(),
        rss: "3065".parse::<i64>().unwrap(),
        rsslim: "18446744073709551615".parse::<i128>().unwrap(),
        startcode: "1".parse::<i64>().unwrap(),
        endcode: "1".parse::<i64>().unwrap(),
        startstack: "0".parse::<i64>().unwrap(),
        kstkesp: "0".parse::<i64>().unwrap(),
        signal: "0".parse::<i64>().unwrap(),
        blocked: "0".parse::<i64>().unwrap(),
        sigignore: "671173123".parse::<i64>().unwrap(),
        sigcatch: "4096".parse::<i64>().unwrap(),
        wchan: "1260".parse::<i64>().unwrap(),
        nswap: "0".parse::<i64>().unwrap(),
        cnswap: "0".parse::<i64>().unwrap(),
        exit_signal: "0".parse::<i64>().unwrap(),
        processor: "17".parse::<i64>().unwrap(),
        rt_priority: "0".parse::<i64>().unwrap(),
        policy: "0".parse::<i64>().unwrap(),
        delayacct_blkio_ticks: "0".parse::<i128>().unwrap(),
        guest_time: "65755103".parse::<i64>().unwrap(),
        cguest_time: "0".parse::<i64>().unwrap(),
        start_data: "0".parse::<i64>().unwrap(),
        end_data: "0".parse::<i64>().unwrap(),
        start_brk: "0".parse::<i64>().unwrap(),
        arg_start: "0".parse::<i64>().unwrap(),
        arg_end: "0".parse::<i64>().unwrap(),
        env_start: "0".parse::<i64>().unwrap(),
        env_end: "0".parse::<i64>().unwrap(),
        exit_code: "0".parse::<i64>().unwrap(),
    };
    assert_eq!(p, test)
}

#[test]
fn test_process_parse_48_kernel() {
    let pid1 = "1 (systemd) S 0 1 1 0 -1 4194560 15260 9952373 112 2320 1425 1853 17764 8432 20 0 1 0 10 38981632 1187 18446744073709551615 1 1 0 0 0 0 671173123 4096 1260 0 0 0 17 1 0 0 7 0 0 0 0 0 0 0 0 0 0";
    let p = Process::parse(pid1.to_string());
    let test = Process {
        pid: "1".parse::<i64>().unwrap(),
        comm: "(systemd)".to_string(),
        state: ProcessState::Sleeping,
        ppid: "0".parse::<i64>().unwrap(),
        pgrp: "1".parse::<i64>().unwrap(),
        session: "1".parse::<i64>().unwrap(),
        tty_nr: "0".parse::<i64>().unwrap(),
        tpgid: "-1".parse::<i64>().unwrap(),
        flags: "4194560".parse::<i64>().unwrap(),
        minflt: "15260".parse::<i64>().unwrap(),
        cminflt: "9952373".parse::<i64>().unwrap(),
        majflt: "112".parse::<i64>().unwrap(),
        cmajflt: "2320".parse::<i64>().unwrap(),
        utime: "1425".parse::<i64>().unwrap(),
        stime: "1853".parse::<i64>().unwrap(),
        cutime: "17764".parse::<i64>().unwrap(),
        cstime: "8432".parse::<i64>().unwrap(),
        priority: "20".parse::<i64>().unwrap(),
        nice: "0".parse::<i64>().unwrap(),
        num_threads: "1".parse::<i64>().unwrap(),
        itrealvalue: "0".parse::<i64>().unwrap(),
        starttime: "10".parse::<i128>().unwrap(),
        vsize: "38981632".parse::<i64>().unwrap(),
        rss: "1187".parse::<i64>().unwrap(),
        rsslim: "18446744073709551615".parse::<i128>().unwrap(),
        startcode: "1".parse::<i64>().unwrap(),
        endcode: "1".parse::<i64>().unwrap(),
        startstack: "0".parse::<i64>().unwrap(),
        kstkesp: "0".parse::<i64>().unwrap(),
        signal: "0".parse::<i64>().unwrap(),
        blocked: "0".parse::<i64>().unwrap(),
        sigignore: "671173123".parse::<i64>().unwrap(),
        sigcatch: "4096".parse::<i64>().unwrap(),
        wchan: "1260".parse::<i64>().unwrap(),
        nswap: "0".parse::<i64>().unwrap(),
        cnswap: "0".parse::<i64>().unwrap(),
        exit_signal: "0".parse::<i64>().unwrap(),
        processor: "17".parse::<i64>().unwrap(),
        rt_priority: "1".parse::<i64>().unwrap(),
        policy: "0".parse::<i64>().unwrap(),
        delayacct_blkio_ticks: "0".parse::<i128>().unwrap(),
        guest_time: "7".parse::<i64>().unwrap(),
        cguest_time: "0".parse::<i64>().unwrap(),
        start_data: "0".parse::<i64>().unwrap(),
        end_data: "0".parse::<i64>().unwrap(),
        start_brk: "0".parse::<i64>().unwrap(),
        arg_start: "0".parse::<i64>().unwrap(),
        arg_end: "0".parse::<i64>().unwrap(),
        env_start: "0".parse::<i64>().unwrap(),
        env_end: "0".parse::<i64>().unwrap(),
        exit_code: "0".parse::<i64>().unwrap(),
    };
    assert_eq!(p, test)
}

#[test]
fn test_process_parse_44_kernel() {
    let pid1 = "1 (init) S 0 1 1 0 -1 4210944 41009 3455307792 28 1892 62 234 12822158 2328772 20 0 1 0 15 36491264 1007 18446744073709551615 1 1 0 0 0 0 0 4096 536962595 0 0 0 17 1 0 0 1 0 0 0 0 0 0 0 0 0 0";
    let p = Process::parse(pid1.to_string());
    let test = Process {
        pid: "1".parse::<i64>().unwrap(),
        comm: "(init)".to_string(),
        state: ProcessState::Sleeping,
        ppid: "0".parse::<i64>().unwrap(),
        pgrp: "1".parse::<i64>().unwrap(),
        session: "1".parse::<i64>().unwrap(),
        tty_nr: "0".parse::<i64>().unwrap(),
        tpgid: "-1".parse::<i64>().unwrap(),
        flags: "4210944".parse::<i64>().unwrap(),
        minflt: "41009".parse::<i64>().unwrap(),
        cminflt: "3455307792".parse::<i64>().unwrap(),
        majflt: "28".parse::<i64>().unwrap(),
        cmajflt: "1892".parse::<i64>().unwrap(),
        utime: "62".parse::<i64>().unwrap(),
        stime: "234".parse::<i64>().unwrap(),
        cutime: "12822158".parse::<i64>().unwrap(),
        cstime: "2328772".parse::<i64>().unwrap(),
        priority: "20".parse::<i64>().unwrap(),
        nice: "0".parse::<i64>().unwrap(),
        num_threads: "1".parse::<i64>().unwrap(),
        itrealvalue: "0".parse::<i64>().unwrap(),
        starttime: "15".parse::<i128>().unwrap(),
        vsize: "36491264".parse::<i64>().unwrap(),
        rss: "1007".parse::<i64>().unwrap(),
        rsslim: "18446744073709551615".parse::<i128>().unwrap(),
        startcode: "1".parse::<i64>().unwrap(),
        endcode: "1".parse::<i64>().unwrap(),
        startstack: "0".parse::<i64>().unwrap(),
        kstkesp: "0".parse::<i64>().unwrap(),
        signal: "0".parse::<i64>().unwrap(),
        blocked: "0".parse::<i64>().unwrap(),
        sigignore: "0".parse::<i64>().unwrap(),
        sigcatch: "4096".parse::<i64>().unwrap(),
        wchan: "536962595".parse::<i64>().unwrap(),
        nswap: "0".parse::<i64>().unwrap(),
        cnswap: "0".parse::<i64>().unwrap(),
        exit_signal: "0".parse::<i64>().unwrap(),
        processor: "17".parse::<i64>().unwrap(),
        rt_priority: "1".parse::<i64>().unwrap(),
        policy: "0".parse::<i64>().unwrap(),
        delayacct_blkio_ticks: "0".parse::<i128>().unwrap(),
        guest_time: "1".parse::<i64>().unwrap(),
        cguest_time: "0".parse::<i64>().unwrap(),
        start_data: "0".parse::<i64>().unwrap(),
        end_data: "0".parse::<i64>().unwrap(),
        start_brk: "0".parse::<i64>().unwrap(),
        arg_start: "0".parse::<i64>().unwrap(),
        arg_end: "0".parse::<i64>().unwrap(),
        env_start: "0".parse::<i64>().unwrap(),
        env_end: "0".parse::<i64>().unwrap(),
        exit_code: "0".parse::<i64>().unwrap(),
    };
    assert_eq!(p, test)
}

#[test]
fn test_is_alive() {
    let t = Process::is_alive(1);
    assert_eq!(t, true)
}

#[test]
#[should_panic]
fn test_process_not_found_panic() {
    Process::fetch("100000000").unwrap();
}

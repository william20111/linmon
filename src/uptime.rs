use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::fmt;

pub static UPTIME: &str = "/proc/uptime";

#[derive(Debug)]
pub struct UpTime {
    uptime: f64,
    idle: f64,
}

impl PartialEq for UpTime {
    fn eq(&self, other: &UpTime) -> bool {
        (self.uptime == other.uptime) & (self.idle == other.idle)
    }
}

impl fmt::Display for UpTime {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "Uptime: {}", self.uptime)
    }
}

impl UpTime {
    pub fn new() -> UpTime {
        let text = UpTime::fetch();
        UpTime::parse(text)
    }

    fn fetch() -> String {
        let path = Path::new(UPTIME);
        let mut data = String::new();
        let mut f = File::open(path).expect("Unable to open file");
        f.read_to_string(&mut data).expect("Unable to read string");
        data
    }

    fn parse(up: String) -> UpTime {
        let mut parse: Vec<&str> = up.split(' ').collect();
        parse.retain(|&i| i != "");
        UpTime {
            uptime: parse[0].parse::<f64>().unwrap(),
            idle: parse[1].parse::<f64>().unwrap(),
        }
    }
}

#[test]
fn test_uptime_parse() {
    let uptime = "1650431.01 1696373.78";
    let u = UpTime {
        uptime: 1650431.01,
        idle: 1696373.78,
    };
    let test = UpTime::parse(uptime.to_string());
    assert_eq!(test, u)
}

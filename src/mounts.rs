use std::fs;
use std::str;
use std::io::{BufReader, Read};
use std::path::Path;

pub static MOUNTS: &str = "/proc/mounts";

#[derive(Debug)]
pub struct Mounts {
    mounts: Vec<Mount>,
}

#[derive(Debug)]
pub struct Mount {
    dev: String,
    mnt: String,
    fs_type: String,
    attrs: String,
    dummy: Vec<String>,
}

impl Mount {
    pub fn is_read_only(&self) -> bool {
        let split: Vec<&str> = self.attrs.split(',').collect();
        split[0] == "ro"
    }

    pub fn get_uuid(&self) -> String {
        if self.dev.contains("dev") {
            for entry in Path::read_dir(Path::new("/dev/disk/by-uuid/")).unwrap() {
                let e = entry.unwrap().path();
                let l = Path::read_link(&e).unwrap();
                let dev = Path::new(&self.dev);
                let dev = dev.strip_prefix("/dev");
                if l.strip_prefix("../../").unwrap().to_str().unwrap()
                    == dev.unwrap().to_str().unwrap()
                {
                    return e.strip_prefix("/dev/disk/by-uuid/")
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();
                }
            }
            return "Not Found".to_string();
        } else {
            return "Not Found".to_string();
        }
    }
}

impl PartialEq for Mount {
    fn eq(&self, other: &Mount) -> bool {
        (self.dev == other.dev) & (self.mnt == other.mnt) & (self.fs_type == other.fs_type)
            & (self.attrs == other.attrs) & (self.dummy == other.dummy)
    }
}

impl Mounts {
    pub fn new() -> Mounts {
        let s = Mounts::fetch();
        Mounts::parse(s)
    }

    fn fetch() -> String {
        let mut s = String::new();
        match fs::File::open(MOUNTS) {
            Ok(f) => {
                let mut buf = BufReader::new(f);
                buf.read_to_string(&mut s).unwrap();
                s
            }
            Err(e) => panic!("{}", e),
        }
    }

    fn parse(s: String) -> Mounts {
        let mut mounts = Mounts { mounts: vec![] };
        for line in s.lines() {
            let line_split: Vec<&str> = line.split_whitespace().collect();
            let m = Mount {
                dev: line_split[0].to_string(),
                mnt: line_split[1].to_string(),
                fs_type: line_split[2].to_string(),
                attrs: line_split[3].to_string(),
                dummy: vec![line_split[4].to_string(), line_split[5].to_string()],
            };
            mounts.mounts.push(m)
        }
        return mounts;
    }
}

#[test]
fn test_get_uuid() {
    let uuid_test = "/dev/nvme0n1p2 /boot ext4 rw,seclabel,relatime,data=ordered 0 0
/dev/nvme0n1p1 /boot/efi vfat rw,relatime,fmask=0077,dmask=0077,codepage=437,iocharset=ascii,shortname=winnt,errors=remount-ro 0 0
selinuxfs /sys/fs/selinux selinuxfs rw,relatime 0 0
";
    let p = Mounts::parse(uuid_test.to_string());
    let m1 = &p.mounts[0];
    let m2 = &p.mounts[1];
    let m3 = &p.mounts[2];
    assert_eq!(
        m1.get_uuid(),
        "46bfd5e8-4a69-4eac-b46b-fcdcce9ee9c9".to_string()
    );
    assert_eq!(m2.get_uuid(), "4AE7-B622");
    assert_eq!(m3.get_uuid(), "Not Found");
}

#[test]
fn test_mounts_read_only_true() {
    let mounts_test = "sysfs /sys sysfs ro,seclabel,nosuid,nodev,noexec,relatime 0 0";
    let p = Mounts::parse(mounts_test.to_string());
    for mount in p.mounts {
        assert_eq!(true, mount.is_read_only())
    }
}

#[test]
fn test_mounts_read_only_false() {
    let mounts_test = "sysfs /sys sysfs rw,seclabel,nosuid,nodev,noexec,relatime 0 0
proc /proc proc rw,nosuid,nodev,noexec,relatime 0 0
devtmpfs /dev devtmpfs rw,seclabel,nosuid,size=8073728k,nr_inodes=2018432,mode=755 0 0";
    let p = Mounts::parse(mounts_test.to_string());
    for mount in p.mounts {
        assert_eq!(false, mount.is_read_only())
    }
}

#[test]
fn test_mounts_parse() {
    let mounts_test = "sysfs /sys sysfs rw,seclabel,nosuid,nodev,noexec,relatime 0 0
proc /proc proc rw,nosuid,nodev,noexec,relatime 0 0
devtmpfs /dev devtmpfs rw,seclabel,nosuid,size=8073728k,nr_inodes=2018432,mode=755 0 0";
    let p = Mounts::parse(mounts_test.to_string());
    let test = Mounts {
        mounts: vec![
            Mount {
                dev: "sysfs".to_string(),
                mnt: "/sys".to_string(),
                fs_type: "sysfs".to_string(),
                attrs: "rw,seclabel,nosuid,nodev,noexec,relatime".to_string(),
                dummy: vec!["0".to_string(), "0".to_string()],
            },
            Mount {
                dev: "proc".to_string(),
                mnt: "/proc".to_string(),
                fs_type: "proc".to_string(),
                attrs: "rw,nosuid,nodev,noexec,relatime".to_string(),
                dummy: vec!["0".to_string(), "0".to_string()],
            },
            Mount {
                dev: "devtmpfs".to_string(),
                mnt: "/dev".to_string(),
                fs_type: "devtmpfs".to_string(),
                attrs: "rw,seclabel,nosuid,size=8073728k,nr_inodes=2018432,mode=755".to_string(),
                dummy: vec!["0".to_string(), "0".to_string()],
            },
        ],
    };
    assert_eq!(p.mounts, test.mounts)
}

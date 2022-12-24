use nix::libc::c_ulong;
use nix::sys::statvfs::statvfs;
use std::fs;

pub fn get_filesystems() -> Vec<Filesystem> {
    get_mount_points()
        .iter()
        .flat_map(|x| filesystem_from_mount_point(x))
        .collect::<Vec<Filesystem>>()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filesystem {
    pub mount_point: String,
    pub size_bytes: c_ulong,
    pub available_bytes: c_ulong,
    pub inodes: c_ulong,
    pub available_inodes: c_ulong,
}

fn get_mount_points() -> Vec<String> {
    fs::read_to_string("/proc/mounts")
        .unwrap()
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>()[1].to_string())
        .collect::<Vec<String>>()
}

fn filesystem_from_mount_point(mount_point: &str) -> Option<Filesystem> {
    match statvfs(mount_point) {
        Ok(stat) => Some(Filesystem {
            mount_point: mount_point.to_string(),
            size_bytes: stat.block_size() * stat.blocks(),
            available_bytes: stat.block_size() * stat.blocks_available(),
            inodes: stat.files(),
            available_inodes: stat.files_available(),
        }),
        Err(_) => None,
    }
}

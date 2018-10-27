extern crate nix;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use std::fs;
use nix::sys::statvfs::statvfs;

pub fn get_filesystems() -> Vec<Filesystem> {
	get_mount_points().iter().map(|x| filesystem_from_mount_point(x) ).flatten().collect::<Vec<Filesystem>>()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filesystem {
	pub mount_point: String,
	pub size_bytes: u64,
	pub available_bytes: u64,
	pub inodes: u64,
	pub available_inodes: u64,
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
			size_bytes: (stat.block_size() * stat.blocks()) as u64,
			available_bytes: (stat.block_size() * stat.blocks_available()) as u64,
			inodes: stat.files() as u64,
			available_inodes: stat.files_available() as u64,
		}),
		Err(_) => None,
	}
}

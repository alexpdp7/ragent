extern crate nix;
extern crate serde;

#[macro_use]
extern crate serde_derive;

pub mod filesystems;
pub mod nagios;
pub mod systemd;

#[derive(Debug, Serialize, Deserialize)]
pub struct RagentInfo {
    pub filesystems: Vec<filesystems::Filesystem>,
    pub units: Vec<systemd::Unit>,
}

pub fn get_ragent_info() -> RagentInfo {
    RagentInfo {
        filesystems: filesystems::get_filesystems(),
        units: systemd::get_units(),
    }
}

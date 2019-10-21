#[macro_use]
extern crate serde_derive;

pub mod entropy;
pub mod filesystems;
pub mod nagios;
pub mod reboot;
pub mod systemd;

#[derive(Debug, Serialize, Deserialize)]
pub struct RagentInfo {
    pub filesystems: Vec<filesystems::Filesystem>,
    pub units: Vec<systemd::Unit>,
    pub reboot: reboot::Reboot,
    pub entropy: usize,
}

pub fn get_ragent_info() -> RagentInfo {
    RagentInfo {
        filesystems: filesystems::get_filesystems(),
        units: systemd::get_units(),
        reboot: reboot::get_reboot(),
        entropy: entropy::get_entropy(),
    }
}

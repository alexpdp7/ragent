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
    pub entropy_available: usize,
    pub entropy_pool_size: usize,
}

pub fn get_ragent_info() -> RagentInfo {
    RagentInfo {
        filesystems: filesystems::get_filesystems(),
        units: systemd::get_units(),
        reboot: reboot::get_reboot(),
        entropy_available: entropy::get_entropy_available(),
        entropy_pool_size: entropy::get_entropy_pool_size(),
    }
}

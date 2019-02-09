use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Reboot {
    pub reboot_required: bool,
}

pub fn get_reboot() -> Reboot {
    Reboot {
        reboot_required: Path::new("/var/run/reboot-required").exists(),
    }
}

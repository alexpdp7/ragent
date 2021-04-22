use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Debug, Serialize, Deserialize)]
pub struct Reboot {
    pub reboot_required: bool,
}

pub fn get_reboot() -> Reboot {
    Reboot {
        reboot_required: check_debian_like_reboot_required()
            || check_el7_reboot_required()
            || check_el8_reboot_required(),
    }
}

fn check_debian_like_reboot_required() -> bool {
    Path::new("/var/run/reboot-required").exists()
}

fn check_el7_reboot_required() -> bool {
    Path::new("/usr/bin/needs-restarting").exists()
        && Command::new("/usr/bin/needs-restarting")
            .arg("-r")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("needs-restarting failed")
            .code()
            .expect("needs-restarting was terminated by a signal")
            == 1
}

fn check_el8_reboot_required() -> bool {
    Path::new("/usr/share/man/man8/dnf-needs-restarting.8.gz").exists()
        && Command::new("/usr/bin/dnf")
            .arg("needs-restarting")
            .arg("-r")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("dnf needs-restarting failed")
            .code()
            .expect("dnf needs-restarting was terminated by a signal")
            == 1
}

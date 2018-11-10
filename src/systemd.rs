use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    pub id: String,
    pub active_state: String,
}

fn execute_command() -> String {
    let out = Command::new("systemctl")
        .arg("show")
        .arg("*.service")
        .arg("*.socket")
        .arg("*.busname")
        .arg("*.target")
        .arg("*.snapshot")
        .arg("*.device")
        .arg("*.mount")
        .arg("*.automount")
        .arg("*.swap")
        .arg("*.timer")
        .arg("*.path")
        .arg("*.slice")
        .arg("*.scope")
        .arg("--property")
        .arg("ActiveState,Id")
        .output()
        .expect("failed");
    String::from_utf8(out.stdout).unwrap()
}

fn get_units_from_output(out: &str) -> Vec<Unit> {
    let mut lines: Vec<&str> = out.lines().collect();
    let mut units = Vec::new();
    while !lines.is_empty() {
        let active_state = lines.pop().unwrap().split('=').nth(1).unwrap();
        let id = lines.pop().unwrap().split('=').nth(1).unwrap();
        let _ = lines.pop().unwrap_or("");
        units.push(Unit {
            id: id.to_string(),
            active_state: active_state.to_string(),
        });
    }
    units
}

pub fn get_units() -> Vec<Unit> {
    let out = execute_command();
    get_units_from_output(&out)
}

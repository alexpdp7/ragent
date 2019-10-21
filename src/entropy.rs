use std::fs;

pub fn get_entropy() -> usize {
    fs::read_to_string("/proc/sys/kernel/random/entropy_avail")
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap()
}

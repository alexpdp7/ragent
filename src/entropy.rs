use std::fs;

pub fn get_entropy_available() -> usize {
    fs::read_to_string("/proc/sys/kernel/random/entropy_avail")
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap()
}

pub fn get_entropy_pool_size() -> usize {
    fs::read_to_string("/proc/sys/kernel/random/poolsize")
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap()
}

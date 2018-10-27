extern crate reqwest;
extern crate ragent;

use ragent::filesystems::Filesystem;
use std::env;
use std::process::exit;
use reqwest::Url;

fn main() {
    match run() {
        Ok(n) => exit(n),
        Err(s) => {
            println!("{}", s);
            exit(3)
        },
    }
}

fn run() -> Result<i32, String> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        return Err("Single parameter must be the URL".to_string());
    }
    let url = match Url::parse(&args[1]) {
        Ok(u) => u,
        Err(_) => return Err(format!("Bad URL {}", &args[1])),
    };
    let mut response = match reqwest::get(url) {
        Ok(r) => r,
        Err(e) => return Err(format!("Error getting URL {}", e.to_string())),
    };
    let result: Vec<Filesystem> = match response.json::<Vec<Filesystem>>() {
        Ok(r) => r,
        Err(e) => return Err(format!("Could not parse {}", e.to_string())),
    };
    print!("RAGENT OK |");
    for filesystem in &result {
        if filesystem.size_bytes != 0 {
            print!(" {}_available_bytes={}B;;;;", filesystem.mount_point, filesystem.available_bytes);
        }
        if filesystem.inodes != 0 {
            print!(" {}_available_inodes={};;;;", filesystem.mount_point, filesystem.available_inodes);
        }
    }
    println!();

    Ok(0)
}

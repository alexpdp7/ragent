extern crate reqwest;
extern crate ragent;

use ragent::filesystems::Filesystem;
use ragent::nagios::NagiosStatus;
use std::env;
use std::process::exit;
use std::error::Error;
use reqwest::Url;

fn main() {
    match run() {
        Ok(n) => exit(n as i32),
        Err(s) => {
            println!("RAGENT UNKNOWN: {}", s);
            exit(NagiosStatus::UNKNOWN as i32)
        },
    }
}

fn run() -> Result<NagiosStatus, Box<Error>> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        return Err(From::from("Single parameter must be the URL"));
    }
    let url = Url::parse(&args[1])?;
    let mut response = reqwest::get(url)?;
    let result: Vec<Filesystem> = response.json::<Vec<Filesystem>>()?;
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

    Ok(NagiosStatus::OK)
}

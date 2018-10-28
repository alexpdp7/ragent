extern crate ragent;
extern crate reqwest;

use ragent::filesystems::Filesystem;
use ragent::nagios::{NagiosMetric, NagiosStatus, NagiosUOM};
use reqwest::Url;
use std::env;
use std::error::Error;
use std::process::exit;

fn main() {
    match run() {
        Ok(n) => exit(n as i32),
        Err(s) => {
            println!("RAGENT UNKNOWN: {}", s);
            exit(NagiosStatus::UNKNOWN as i32)
        }
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
            let metric = NagiosMetric::<u64> {
                label: format!("{}_available_bytes", filesystem.mount_point),
                uom: NagiosUOM::Bytes,
                value: filesystem.available_bytes,
                warn: None,
                crit: None,
                min: Some(0),
                max: Some(filesystem.size_bytes),
            };
            print!(" {}", metric);
        }
        if filesystem.inodes != 0 {
            let metric = NagiosMetric::<u64> {
                label: format!("{}_available_inodes", filesystem.mount_point),
                uom: NagiosUOM::NoUnit,
                value: filesystem.available_inodes,
                warn: None,
                crit: None,
                min: Some(0),
                max: Some(filesystem.inodes),
            };
            print!(" {}", metric);
        }
    }
    println!();

    Ok(NagiosStatus::OK)
}

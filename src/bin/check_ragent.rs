extern crate ragent;
extern crate reqwest;

use ragent::filesystems::Filesystem;
use ragent::nagios::{HasNagiosStatus, NagiosMetric, NagiosStatus, NagiosUOM};
use reqwest::Url;
use std::env;
use std::error::Error;
use std::process::exit;
use std::vec::Vec;

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
    let filesystems: Vec<Filesystem> = response.json::<Vec<Filesystem>>()?;
    let mut metrics: Vec<Box<HasNagiosStatus>> = Vec::new();
    for filesystem in &filesystems {
        if filesystem.size_bytes != 0 {
            metrics.push(Box::new(NagiosMetric::<u64> {
                label: format!("{}_available_bytes", filesystem.mount_point),
                uom: NagiosUOM::Bytes,
                value: filesystem.available_bytes,
                warn: Some(::std::cmp::min(
                    filesystem.size_bytes / 5,
                    2 * 1024 * 1024 * 1024,
                )),
                crit: Some(::std::cmp::min(
                    filesystem.size_bytes / 10,
                    1024 * 1024 * 1024,
                )),
                min: Some(0),
                max: Some(filesystem.size_bytes),
            }));
        }
        if filesystem.inodes != 0 {
            metrics.push(Box::new(NagiosMetric::<u64> {
                label: format!("{}_available_inodes", filesystem.mount_point),
                uom: NagiosUOM::NoUnit,
                value: filesystem.available_inodes,
                warn: Some(filesystem.inodes / 5),
                crit: Some(filesystem.inodes / 10),
                min: Some(0),
                max: Some(filesystem.inodes),
            }));
        }
    }

    let status = (&metrics)
        .into_iter()
        .map(|m| m.get_status())
        .fold(NagiosStatus::OK, ::std::cmp::max);

    print!("RAGENT {:?} |", status);

    for metric in &metrics {
        print!(" {}", metric);
    }

    println!();

    Ok(status)
}

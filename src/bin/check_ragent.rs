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
    exit(match run() {
        Ok(status) => status,
        Err(s) => {
            println!("RAGENT UNKNOWN: {}", s);
            NagiosStatus::UNKNOWN
        }
    } as i32);
}

fn get_url() -> Result<Url, Box<Error>> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        return Err(From::from("Single parameter must be the URL"));
    }
    Ok(Url::parse(&args[1]).map_err(|e| format!("Invalid URL {}: {}", args[1], e))?)
}

fn get_from_agent(url: Url) -> Result<Vec<Filesystem>, Box<Error>> {
    let mut response = reqwest::get(url)?;
    Ok(response.json::<Vec<Filesystem>>().map_err(|e| format!("Could not parse JSON from {:?}: {1}", response, e))?)
}

fn get_metrics(filesystems: &[Filesystem]) -> Vec<Box<HasNagiosStatus>> {
    let mut metrics: Vec<Box<HasNagiosStatus>> = Vec::new();
    for filesystem in filesystems.iter() {
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
    metrics
}

fn make_nagios(metrics: &[Box<HasNagiosStatus>]) -> NagiosStatus {
    let status = metrics
        .iter()
        .map(|m| m.get_status())
        .fold(NagiosStatus::OK, ::std::cmp::max);

    print!("RAGENT {:?} |", status);

    for metric in metrics.iter() {
        print!(" {}", metric);
    }

    println!();

    status
}

fn run() -> Result<NagiosStatus, Box<Error>> {
    let url = get_url()?;
    let filesystems = get_from_agent(url)?;
    let metrics = get_metrics(&filesystems);
    Ok(make_nagios(&metrics))
}

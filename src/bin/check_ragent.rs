use reqwest;

use ragent::filesystems::Filesystem;
use ragent::nagios::{HasNagiosStatus, NagiosMetric, NagiosStatus, NagiosUOM};
use ragent::systemd::Unit;
use ragent::RagentInfo;
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

fn get_url() -> Result<Url, Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        return Err(From::from("Single parameter must be the URL"));
    }
    Ok(Url::parse(&args[1]).map_err(|e| format!("Invalid URL {}: {}", args[1], e))?)
}

fn get_from_agent(url: Url) -> Result<RagentInfo, Box<dyn Error>> {
    let mut response = reqwest::get(url)?;
    Ok(response
        .json::<RagentInfo>()
        .map_err(|e| format!("Could not parse JSON from {:?}: {1}", response, e))?)
}

fn get_metrics(filesystems: &[Filesystem]) -> Vec<Box<dyn HasNagiosStatus>> {
    let mut metrics: Vec<Box<dyn HasNagiosStatus>> = Vec::new();
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

fn make_nagios(metrics: &[Box<dyn HasNagiosStatus>], units: &[Unit]) -> NagiosStatus {
    let metrics_status = metrics
        .iter()
        .map(|m| m.get_status())
        .fold(NagiosStatus::OK, ::std::cmp::max);

    let failed_units: Vec<&Unit> = units
        .iter()
        .filter(|u| u.active_state == "failed")
        .collect();
    let unit_status = if failed_units.is_empty() {
        NagiosStatus::OK
    } else {
        NagiosStatus::CRITICAL
    };

    let status = ::std::cmp::max(metrics_status, unit_status);

    print!("RAGENT {:?}", status);

    if !failed_units.is_empty() {
        print!(
            " FAILED UNITS {:?}",
            failed_units.iter().map(|u| &u.id).collect::<Vec<&String>>()
        );
    }

    print!(" |");

    for metric in metrics.iter() {
        print!(" {}", metric);
    }

    println!();

    status
}

fn run() -> Result<NagiosStatus, Box<dyn Error>> {
    let url = get_url()?;
    let ragent_info = get_from_agent(url)?;
    let metrics = get_metrics(&ragent_info.filesystems);
    Ok(make_nagios(&metrics, &ragent_info.units))
}

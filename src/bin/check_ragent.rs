use clap::Parser;
use ragent::nagios::{get_worst_status, HasNagiosStatus, NagiosMetric, NagiosStatus, NagiosUom};
use ragent::systemd::Unit;
use ragent::{get_ragent_info, RagentInfo};
use reqwest::Url;
use std::error::Error;
use std::process::exit;
use std::vec::Vec;

#[derive(Parser, Debug)]
#[clap(about = "Nagios check for ragent")]
struct Args {
    /// If provided (like http://host:21488/), contacts a remote ragent daemon. Else check local host.
    url: Option<Url>,
    #[clap(long)]
    warning_units: Vec<String>,
}

fn main() {
    exit(match run() {
        Ok(status) => status,
        Err(s) => {
            println!("RAGENT UNKNOWN: {}", s);
            NagiosStatus::Unknown
        }
    } as i32);
}

fn get_from_agent(url: Url) -> Result<RagentInfo, Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?;
    Ok(response
        .json::<RagentInfo>()
        .map_err(|e| format!("Could not parse JSON: {}", e))?)
}

fn get_metrics(ragent_info: &RagentInfo) -> Vec<Box<dyn HasNagiosStatus>> {
    let mut metrics: Vec<Box<dyn HasNagiosStatus>> = Vec::new();
    for filesystem in ragent_info.filesystems.iter() {
        if filesystem.size_bytes != 0 {
            metrics.push(Box::new(NagiosMetric::<u64> {
                label: format!("{}_available_bytes", filesystem.mount_point),
                uom: NagiosUom::Bytes,
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
                uom: NagiosUom::NoUnit,
                value: filesystem.available_inodes,
                warn: Some(filesystem.inodes / 5),
                crit: Some(filesystem.inodes / 10),
                min: Some(0),
                max: Some(filesystem.inodes),
            }));
        }
    }
    metrics.push(Box::new(NagiosMetric::<usize> {
        label: "entropy".to_string(),
        uom: NagiosUom::NoUnit,
        value: ragent_info.entropy_available,
        warn: Some(ragent_info.entropy_pool_size / 2),
        crit: Some(ragent_info.entropy_pool_size / 4),
        min: Some(0),
        max: None,
    }));
    metrics
}

fn make_nagios(
    metrics: &[Box<dyn HasNagiosStatus>],
    ragent_info: RagentInfo,
    warning_units: Vec<String>,
) -> NagiosStatus {
    let metrics_status = get_worst_status(
        &metrics
            .iter()
            .map(|m| m.get_status())
            .collect::<Vec<NagiosStatus>>(),
    );

    let failed_warning_units: Vec<&Unit> = ragent_info
        .units
        .iter()
        .filter(|u| u.active_state == "failed")
        .filter(|u| warning_units.contains(&u.id))
        .collect();

    let failed_critical_units: Vec<&Unit> = ragent_info
        .units
        .iter()
        .filter(|u| u.active_state == "failed")
        .filter(|u| !warning_units.contains(&u.id))
        .collect();

    let unit_status = if !failed_critical_units.is_empty() {
        NagiosStatus::Critical
    } else if !failed_warning_units.is_empty() {
        NagiosStatus::Warning
    } else {
        NagiosStatus::Ok
    };

    let reboot_status = if ragent_info.reboot.reboot_required {
        NagiosStatus::Warning
    } else {
        NagiosStatus::Ok
    };

    let status = get_worst_status(&[metrics_status, unit_status, reboot_status]);

    print!("RAGENT {}", status);

    if !failed_warning_units.is_empty() {
        print!(
            " FAILED WARNING UNITS {:?}",
            failed_warning_units
                .iter()
                .map(|u| &u.id)
                .collect::<Vec<&String>>()
        );
    }

    if !failed_critical_units.is_empty() {
        print!(
            " FAILED CRITICAL UNITS {:?}",
            failed_critical_units
                .iter()
                .map(|u| &u.id)
                .collect::<Vec<&String>>()
        );
    }

    if ragent_info.reboot.reboot_required {
        print!(" REBOOT REQUIRED");
    }

    for metric in metrics.iter() {
        if metric.get_status() != NagiosStatus::Ok {
            print!(" {}", metric.get_display_status())
        }
    }

    print!(" |");

    for metric in metrics.iter() {
        print!(" {}", metric);
    }

    println!();

    status
}

fn run() -> Result<NagiosStatus, Box<dyn Error>> {
    let args = Args::parse();
    let ragent_info = match args.url {
        Some(url) => get_from_agent(url)?,
        None => get_ragent_info(),
    };
    let metrics = get_metrics(&ragent_info);
    Ok(make_nagios(&metrics, ragent_info, args.warning_units))
}

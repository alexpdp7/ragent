use std::fmt;

use nagios_range::NagiosRange;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NagiosStatus {
    Ok,
    Warning,
    Critical,
    Unknown,
}

impl fmt::Display for NagiosStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{self:?}").to_uppercase())
    }
}

pub fn get_worst_status(statuses: &[NagiosStatus]) -> NagiosStatus {
    *statuses.iter().max().unwrap_or(&NagiosStatus::Ok)
}

#[derive(Clone)]
pub struct NagiosMetric {
    pub label: String,
    pub uom: NagiosUom,
    pub value: f64,
    pub warn: Option<NagiosRange>,
    pub crit: Option<NagiosRange>,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

fn or_empty<T: fmt::Display + Clone>(v: Option<T>) -> String {
    match v {
        Some(n) => n.to_string(),
        None => "".to_string(),
    }
}

pub trait HasNagiosStatus: ::std::fmt::Display {
    fn get_status(&self) -> NagiosStatus;
    fn get_display_status(&self) -> String;
}

impl HasNagiosStatus for NagiosMetric
where
    NagiosMetric: ::std::fmt::Display,
{
    fn get_status(&self) -> NagiosStatus {
        if let Some(crit) = &self.crit {
            if crit.check(self.value) {
                return NagiosStatus::Critical;
            }
        }
        if let Some(warn) = &self.warn {
            if warn.check(self.value) {
                return NagiosStatus::Warning;
            }
        }
        NagiosStatus::Ok
    }

    fn get_display_status(&self) -> String {
        format!("{} is {:?}", self.label, self.get_status())
    }
}

impl fmt::Display for NagiosMetric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "'{}'={}{};{};{};{};{}",
            self.label,
            self.value,
            self.uom,
            or_empty(self.warn),
            or_empty(self.crit),
            or_empty(self.min),
            or_empty(self.max)
        )
    }
}

#[derive(Clone)]
pub enum NagiosUom {
    NoUnit,
    Seconds,
    Percentage,
    Bytes,
    Counter,
}

impl fmt::Display for NagiosUom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            NagiosUom::NoUnit => "",
            NagiosUom::Seconds => "s",
            NagiosUom::Percentage => "%",
            NagiosUom::Bytes => "B",
            NagiosUom::Counter => "c",
        })
    }
}

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NagiosStatus {
    Ok,
    Warning,
    Critical,
    Unknown,
}

impl fmt::Display for NagiosStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_uppercase())
    }
}

pub fn get_worst_status(statuses: &[NagiosStatus]) -> NagiosStatus {
    *statuses.iter().max().unwrap_or(&NagiosStatus::Ok)
}

#[derive(Clone)]
pub struct NagiosMetric<T: ::std::cmp::Ord + Clone> {
    pub label: String,
    pub uom: NagiosUom,
    pub value: T,
    pub warn: Option<T>,
    pub crit: Option<T>,
    pub min: Option<T>,
    pub max: Option<T>,
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

impl<T: ::std::cmp::Ord + Clone> HasNagiosStatus for NagiosMetric<T>
where
    NagiosMetric<T>: ::std::fmt::Display,
{
    fn get_status(&self) -> NagiosStatus {
        if let Some(crit) = &self.crit {
            if self.value <= *crit {
                return NagiosStatus::Critical;
            }
        }
        if let Some(warn) = &self.warn {
            if self.value <= *warn {
                return NagiosStatus::Warning;
            }
        }
        NagiosStatus::Ok
    }

    fn get_display_status(&self) -> String {
        format!("{} is {:?}", self.label, self.get_status())
    }
}

impl<T: fmt::Display + Clone> fmt::Display for NagiosMetric<T>
where
    T: std::cmp::Ord,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "'{}'={}{};{};{};{};{}",
            self.label,
            self.value,
            self.uom,
            or_empty(self.warn.clone()),
            or_empty(self.crit.clone()),
            or_empty(self.min.clone()),
            or_empty(self.max.clone())
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

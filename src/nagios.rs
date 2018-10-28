use std::fmt;

pub enum NagiosStatus {
    OK,
    WARNING,
    CRITICAL,
    UNKNOWN,
}

pub struct NagiosMetric<T> {
    pub label: String,
    pub uom: NagiosUOM,
    pub value: T,
    pub warn: Option<T>,
    pub crit: Option<T>,
    pub min: Option<T>,
    pub max: Option<T>,
}

fn or_empty(v: Option<u64>) -> String {
    match v {
        Some(n) => n.to_string(),
        None => "".to_string(),
    }
}

impl fmt::Display for NagiosMetric<u64> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

pub enum NagiosUOM {
    NoUnit,
    Seconds,
    Percentage,
    Bytes,
    Counter,
}

impl fmt::Display for NagiosUOM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            NagiosUOM::NoUnit => "",
            NagiosUOM::Seconds => "s",
            NagiosUOM::Percentage => "%",
            NagiosUOM::Bytes => "b",
            NagiosUOM::Counter => "c",
        })
    }
}

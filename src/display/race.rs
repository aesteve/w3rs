use crate::race::Race;
use std::fmt;
use std::fmt::{Display, Formatter};

impl Display for Race {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let short = match self {
            Race::Human => "HU",
            Race::Orc => "OR",
            Race::NightElf => "NE",
            Race::Undead => "UD",
            Race::Random => "??",
            Race::Unknown => "N/A",
        };
        write!(f, "{}", short)
    }
}

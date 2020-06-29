#[derive(Debug, PartialEq, Clone)]
pub enum Race {
    Human,
    Orc,
    NightElf,
    Undead,
    Random,
    Unknown,
}

impl Race {
    pub(crate) fn from_u8(flag: u8) -> Race {
        match flag {
            1 => Race::Human,
            65 => Race::Human,
            2 => Race::Orc,
            66 => Race::Orc,
            4 => Race::NightElf,
            68 => Race::NightElf,
            8 => Race::Undead,
            72 => Race::Undead,
            32 => Race::Random,
            96 => Race::Random,
            _ => Race::Unknown,
        }
    }
}

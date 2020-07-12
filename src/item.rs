#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    // OR (Voodoo Lounge)
    HealingSalve,
    ScrollOfSpeed,
    LesserClarityPotion,
    PotionOfHealing,
    PotionOfMana,
    ScrollOfTownPortal,
    OrbOfLightning,
    TinyGreatHall,
}
impl Item {
    pub(crate) fn from_str(str: &str) -> Option<Item> {
        match str {
            "hslv" => Some(Item::HealingSalve),
            "shas" => Some(Item::ScrollOfSpeed),
            _ => None,
        }
    }
}

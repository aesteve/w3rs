#[derive(Debug, PartialEq, Clone)]
pub enum Building {
    // NE
    TreeOfLife,
    TreeOfAges,
    TreeOfEternity,
    AltarOfElders,
    MoonWell,
    AncientOfWar,
    AncientOfWonders,
    HuntersHall,
    AncientProtector,
    AncientOfWind,
    AncientOfLore,
    ChimaeraRoot,
    // OR
    GreatHall,
    StrongHold,
    Fortress,
    AltarOfStorms,
    Burrow,
    ReinforcedBurrow,
    Barracks,
    WarMill,
    WatchTower,
    VoodooLounge,
    SpiritLodge,
    Beastiary,
    TaurenTotem,
    // Neutral
    Tavern,
    MercernaryCamp,
}

impl Building {
    pub(crate) fn from_str(str: &str) -> Option<Building> {
        match str {
            // NE
            "etol" => Some(Building::TreeOfLife),
            "etoa" => Some(Building::TreeOfAges),
            "etoe" => Some(Building::TreeOfEternity),
            "eate" => Some(Building::AltarOfElders),
            "emow" => Some(Building::MoonWell),
            "eaom" => Some(Building::AncientOfWar),
            "eden" => Some(Building::AncientOfWonders),
            "edob" => Some(Building::HuntersHall),
            "etrp" => Some(Building::AncientProtector),
            "eaow" => Some(Building::AncientOfWind),
            "eaoe" => Some(Building::AncientOfLore),
            "edos" => Some(Building::ChimaeraRoot),
            // OR
            "ogre" => Some(Building::GreatHall),
            "ostr" => Some(Building::StrongHold),
            "ofrt" => Some(Building::Fortress),
            "oalt" => Some(Building::AltarOfStorms),
            "otrb" => Some(Building::Burrow),
            "orbr" => Some(Building::ReinforcedBurrow),
            "obar" => Some(Building::Barracks),
            "ofor" => Some(Building::WarMill),
            "owtw" => Some(Building::WatchTower),
            "ovln" => Some(Building::VoodooLounge),
            "osld" => Some(Building::SpiritLodge),
            "obea" => Some(Building::Beastiary),
            "otto" => Some(Building::TaurenTotem),
            // Neutral
            "nmer" => Some(Building::MercernaryCamp),
            "ntav" => Some(Building::Tavern),
            _ => None,
        }
    }
}

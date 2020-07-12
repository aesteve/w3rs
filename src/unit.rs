#[derive(Debug, PartialEq, Clone)]
pub enum Unit {
    // NE
    Wisp,
    Archer,
    Huntress,
    GlaiveThrower,
    Dryad,
    DruidOfTheClaw,
    MountainGiant,
    DruidOfTheTalon,
    FaerieDragon,
    HippogryphRider,
    Chimaera,
    Treant,
    OwlScout,
    AvatarOfVengeance,
    SpiritOfVengeance,
    // OR
    Peon,
    Grunt,
    HeadHunter,
    Berserker,
    Demolisher,
    Shaman,
    WitchDoctor,
    SpiritWalker,
    Raider,
    KodoBeast,
    WindRinder,
    TrollBatrider,
    Tauren,
    SpiritWolf,
    DireWolf,
    ShadowWolf,
    // Neutral
    ForestTrollShadowPriest,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Hero {
    // NE
    DemonHunter,
    KeeperOfTheGrove,
    PriestessOfTheMoon,
    Warden,
    // OR
    BladeMaster,
    FarSeer,
    TaurenChieftain,
    ShadowHunter,
    // Neutral
    NagaSeaWitch,
    DarkRanger,
    PandarenBrewmaster,
    Beastmaster,
    PitLord,
    GoblinTinker,
    FireLord,
    GoblinAlchemist,
}

impl Hero {
    pub(crate) fn from_str(str: &str) -> Option<Hero> {
        match str {
            "Edem" => Some(Hero::DemonHunter),
            "Ofar" => Some(Hero::FarSeer),
            "Otch" => Some(Hero::TaurenChieftain),
            "Obla" => Some(Hero::BladeMaster),
            "Npbm" => Some(Hero::PandarenBrewmaster),
            _ => None,
        }
    }
}

impl Unit {
    pub(crate) fn from_str(str: &str) -> Option<Unit> {
        match str {
            "ewsp" => Some(Unit::Wisp),
            "earc" => Some(Unit::Archer),
            "esen" => Some(Unit::Huntress),
            "edry" => Some(Unit::Dryad),
            "edoc" => Some(Unit::DruidOfTheClaw),
            "opeo" => Some(Unit::Peon),
            "ohun" => Some(Unit::HeadHunter),
            "odoc" => Some(Unit::WitchDoctor),
            "otbk" => Some(Unit::Berserker),
            "nfsp" => Some(Unit::ForestTrollShadowPriest),
            _ => None,
        }
    }
}

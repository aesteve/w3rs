// Hero spells: Trainable
#[derive(Debug, PartialEq, Clone)]
pub enum HeroSpell {
    // NE
    // Edem
    ManaBurn,
    Immolation,
    Evasion,
    Metamorphosis,
    //
    EntanglingRoots,
    ForceOfNature,
    ThornsAura,
    Tranquility,
    //
    Scout,
    SearingArrows,
    TrueshotAura,
    Starfall,
    //
    FanOfKnives,
    Blink,
    ShadowStrike,
    Vengeance,
    // OR
    // Obla
    WindWalk,
    MirrorImage,
    CriticalStrike,
    Bladestorm,
    // Ofar
    FarSight,
    ChainLightning,
    FeralSpirit,
    EarthQuake,
    // Otch
    ShockWave,
    WarStomp,
    EnduranceAura,
    Reincarnation,
    //
    HealingWave,
    Hex,
    SerpentWard,
    BigBadVoodoo,
    // Neutral
    // Nnsw?
    ForkedLightning,
    FrostArrows,
    ManaShield,
    Tornado,
    // Ndrg?
    Silence,
    BlackArrow,
    LifeDrain,
    Charm,
    // Npbm
    BreathOfFire,
    DrunkenHaze,
    DrunkenBrawler,
    StormEarthAndFire,
    // Nbmr?
    SummonBear,
    SummonQuilbeast,
    SummonHawk,
    Stampede,
    // Npld?
    RainOfFire,
    HowlOfTerror,
    CleavingAttack,
    Doom,
    // Ngtk? Ngtr?
    PocketFactory,
    ClusterRockets,
    EngineeringUpgrade,
    RoboGoblin,
    // Nfld?
    SoulBurn,
    SummonLavaSpawn,
    Incinerate,
    Volcano,
    // Galc?
    HealingSpray,
    ChemicalRage,
    AcidBomb,
    Transmute,
}

impl HeroSpell {
    pub(crate) fn from_str(str: &str) -> Option<HeroSpell> {
        match str {
            "AEmb" => Some(HeroSpell::ManaBurn),
            "AOcl" => Some(HeroSpell::ChainLightning),
            "AOsf" => Some(HeroSpell::FeralSpirit),
            "AOMi" => Some(HeroSpell::MirrorImage),
            "AOae" => Some(HeroSpell::EnduranceAura),
            "AOws" => Some(HeroSpell::WarStomp),
            "ANbf" => Some(HeroSpell::BreathOfFire),
            "ANdb" => Some(HeroSpell::DrunkenBrawler),
            _ => None,
        }
    }

    pub(crate) fn from_bin(binary: [u8; 2]) -> Option<HeroSpell> {
        match binary {
            [157, 0] => Some(HeroSpell::ShockWave),
            [158, 0] => Some(HeroSpell::FeralSpirit),
            [159, 0] => Some(HeroSpell::WarStomp),
            [160, 0] => Some(HeroSpell::Bladestorm),
            [161, 0] => Some(HeroSpell::WindWalk),

            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnitSpell {
    ReturnResources,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Spell {
    Hero(HeroSpell),
    Unit(UnitSpell),
}

impl UnitSpell {
    pub(crate) fn from_bin(binary: [u8; 2]) -> Option<UnitSpell> {
        match binary {
            [49, 0] => Some(UnitSpell::ReturnResources),
            _ => None,
        }
    }
}

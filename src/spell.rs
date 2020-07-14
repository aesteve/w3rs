// Hero spells: Trainable
#[derive(Debug, PartialEq, Clone)]
pub enum HeroSpell {
    // NE
    // Edem
    ManaBurn,
    Immolation,
    ImmolationOn,
    ImmolationOff,
    Evasion,
    Metamorphosis,
    // Ekee
    EntanglingRoots,
    ForceOfNature,
    ThornsAura,
    Tranquility,
    // Emoo
    Scout,
    SearingArrows,
    EnableAutoSearingArrows,
    DisableAutoSearingArrows,
    TrueshotAura,
    Starfall,
    // Ewar
    FanOfKnives,
    Blink,
    ShadowStrike,
    SpiritOfVengeance,
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
    // Oshd
    HealingWave,
    Hex,
    SerpentWard,
    BigBadVoodoo,
    // Udea
    DeathCoil,
    DeathPact,
    UnholyAura,
    AnimateDead,
    // Udrea
    CarrionSwarm,
    Sleep,
    VampiricAura,
    Inferno,
    // Ulic
    FrostNova,
    FrostArmor,
    EnableAutoFrostArmor,
    DisableAutoFrostArmor,
    DarkRitual,
    DeathAndDecay,
    // Ucrl
    Impale,
    SpikedCarapace,
    CarrionBeetles,
    LocustSwarm,
    // HU
    // Hamg
    Blizzard,
    SummonWaterElemental,
    BrillianceAura,
    MassTeleport,
    // Hmkg
    StormBolt,
    ThunderClap,
    Bash,
    Avatar,
    // Hpal
    HolyLight,
    DivineShield,
    DivineShieldOff,
    DevotionAura,
    Resurrection,
    // Hblm
    SiphonMana,
    FlameStrike,
    Banish,
    SummonPhoenix,
    // Neutral
    // Nnsw
    ForkedLightning,
    FrostArrows,
    ManaShield,
    Tornado,
    // Nbrn
    Silence,
    BlackArrow,
    LifeDrain,
    Charm,
    // Npbm
    BreathOfFire,
    DrunkenHaze,
    DrunkenBrawler,
    StormEarthAndFire,
    // Nbst
    SummonBear,
    SummonQuilbeast,
    SummonHawk,
    Stampede,
    // Nplh
    RainOfFire,
    HowlOfTerror,
    CleavingAttack,
    Doom,
    // Ntin
    PocketFactory,
    ClusterRockets,
    EngineeringUpgrade,
    RoboGoblin,
    // Nfir
    SoulBurn,
    SummonLavaSpawn,
    Incinerate,
    Volcano,
    // Nalc
    HealingSpray,
    ChemicalRage,
    AcidBomb,
    Transmute,
}

impl HeroSpell {
    pub(crate) fn from_str(str: &str) -> Option<HeroSpell> {
        match str {
            // NE
            // Edem
            "AEmb" => Some(HeroSpell::ManaBurn),
            "AEim" => Some(HeroSpell::Immolation),
            "AEev" => Some(HeroSpell::Evasion),
            "AEme" => Some(HeroSpell::Metamorphosis),
            // Ekee
            "AEer" => Some(HeroSpell::EntanglingRoots),
            "AEfn" => Some(HeroSpell::ForceOfNature),
            "AEah" => Some(HeroSpell::ThornsAura),
            "AEtq" => Some(HeroSpell::Tranquility),
            // Emoo
            "AEst" => Some(HeroSpell::Scout),
            "AHfa" => Some(HeroSpell::SearingArrows),
            "AEar" => Some(HeroSpell::TrueshotAura),
            "AEsf" => Some(HeroSpell::Starfall),
            // Ewar
            "AEbl" => Some(HeroSpell::Blink),
            "AEfk" => Some(HeroSpell::FanOfKnives),
            "AEsh" => Some(HeroSpell::ShadowStrike),
            "AEsv" => Some(HeroSpell::SpiritOfVengeance),
            // OR
            // Obla
            "AOMi" => Some(HeroSpell::MirrorImage),
            "AOCr" => Some(HeroSpell::CriticalStrike),
            "AOwk" => Some(HeroSpell::WindWalk),
            "AOww" => Some(HeroSpell::Bladestorm),
            // Ofar
            "AOcl" => Some(HeroSpell::ChainLightning),
            "AOfs" => Some(HeroSpell::FarSight),
            "AOsf" => Some(HeroSpell::FeralSpirit),
            "AOeq" => Some(HeroSpell::EarthQuake),
            // Otch
            "AOsh" => Some(HeroSpell::ShockWave),
            "AOae" => Some(HeroSpell::EnduranceAura),
            "AOws" => Some(HeroSpell::WarStomp),
            "AOre" => Some(HeroSpell::Reincarnation),
            // Oshd
            "AOhw" => Some(HeroSpell::HealingWave),
            "AOhx" => Some(HeroSpell::Hex),
            "AOsw" => Some(HeroSpell::SerpentWard),
            "AOvd" => Some(HeroSpell::BigBadVoodoo),
            // UD
            // Udea
            "AUdc" => Some(HeroSpell::DeathCoil),
            "AUdp" => Some(HeroSpell::DeathPact),
            "AUau" => Some(HeroSpell::UnholyAura),
            "AUan" => Some(HeroSpell::AnimateDead),
            // Udre
            "AUcs" => Some(HeroSpell::CarrionSwarm),
            "AUsl" => Some(HeroSpell::Sleep),
            "AUav" => Some(HeroSpell::VampiricAura),
            "AUin" => Some(HeroSpell::Inferno),
            // Ulic
            "AUfn" => Some(HeroSpell::FrostNova),
            "AUfa" => Some(HeroSpell::FrostArmor),
            "AUfu" => Some(HeroSpell::FrostArmor),
            "AUdr" => Some(HeroSpell::DarkRitual),
            "AUdd" => Some(HeroSpell::DeathAndDecay),
            // Ucrl
            "AUim" => Some(HeroSpell::Impale),
            "AUts" => Some(HeroSpell::SpikedCarapace),
            "AUcb" => Some(HeroSpell::CarrionBeetles),
            "AUls" => Some(HeroSpell::LocustSwarm),
            // HU
            // Hamg
            "AHbz" => Some(HeroSpell::Blizzard),
            "AHwe" => Some(HeroSpell::SummonWaterElemental),
            "AHab" => Some(HeroSpell::BrillianceAura),
            "AHmt" => Some(HeroSpell::MassTeleport),
            // Hmkg
            "AHtb" => Some(HeroSpell::StormBolt),
            "AHtc" => Some(HeroSpell::ThunderClap),
            "AHbh" => Some(HeroSpell::Bash),
            "AHav" => Some(HeroSpell::Avatar),
            // Hpal
            "AHhb" => Some(HeroSpell::HolyLight),
            "AHds" => Some(HeroSpell::DivineShield),
            "AHad" => Some(HeroSpell::DevotionAura),
            "AHre" => Some(HeroSpell::Resurrection),
            // Hblm
            "AHdr" => Some(HeroSpell::SiphonMana),
            "AHfs" => Some(HeroSpell::FlameStrike),
            "AHbn" => Some(HeroSpell::Banish),
            "AHpx" => Some(HeroSpell::SummonPhoenix),
            // Neutral
            // Npbm
            "ANbf" => Some(HeroSpell::BreathOfFire),
            "ANdb" => Some(HeroSpell::DrunkenBrawler),
            "ANdh" => Some(HeroSpell::DrunkenHaze),
            "ANef" => Some(HeroSpell::StormEarthAndFire),
            // Nbrn
            "ANdr" => Some(HeroSpell::LifeDrain),
            "ANsi" => Some(HeroSpell::Silence),
            "ANba" => Some(HeroSpell::BlackArrow),
            "ANch" => Some(HeroSpell::Charm),
            // Nnsw
            "ANms" => Some(HeroSpell::ManaShield),
            "ANfa" => Some(HeroSpell::FrostArrows),
            "ANfl" => Some(HeroSpell::ForkedLightning),
            "ANto" => Some(HeroSpell::Tornado),
            // Nplh
            "ANrf" => Some(HeroSpell::RainOfFire),
            "ANca" => Some(HeroSpell::CleavingAttack),
            "ANht" => Some(HeroSpell::HowlOfTerror),
            "ANdo" => Some(HeroSpell::Doom),
            // Nbst
            "ANsg" => Some(HeroSpell::SummonBear),
            "ANsq" => Some(HeroSpell::SummonQuilbeast),
            "ANsw" => Some(HeroSpell::SummonHawk),
            "ANst" => Some(HeroSpell::Stampede),
            // Ntin
            "ANeg" => Some(HeroSpell::EngineeringUpgrade),
            "ANcs" => Some(HeroSpell::ClusterRockets),
            "ANc1" => Some(HeroSpell::ClusterRockets),
            "ANc2" => Some(HeroSpell::ClusterRockets),
            "ANc3" => Some(HeroSpell::ClusterRockets),
            "ANsy" => Some(HeroSpell::PocketFactory),
            "ANs1" => Some(HeroSpell::PocketFactory),
            "ANs2" => Some(HeroSpell::PocketFactory),
            "ANs3" => Some(HeroSpell::PocketFactory),
            "ANrg" => Some(HeroSpell::RoboGoblin),
            "ANg1" => Some(HeroSpell::RoboGoblin),
            "ANg2" => Some(HeroSpell::RoboGoblin),
            "ANg3" => Some(HeroSpell::RoboGoblin),
            // Nfir
            "ANic" => Some(HeroSpell::Incinerate),
            "ANia" => Some(HeroSpell::Incinerate),
            "ANso" => Some(HeroSpell::SoulBurn),
            "ANlm" => Some(HeroSpell::SummonLavaSpawn),
            "ANvc" => Some(HeroSpell::Volcano),
            // Nalc
            "ANhs" => Some(HeroSpell::HealingSpray),
            "ANab" => Some(HeroSpell::AcidBomb),
            "ANcr" => Some(HeroSpell::ChemicalRage),
            "ANtm" => Some(HeroSpell::Transmute),
            _ => None,
        }
    }

    pub(crate) fn from_bin(binary: [u8; 2]) -> Option<HeroSpell> {
        match binary {
            // HU
            [118, 0] => Some(HeroSpell::Avatar),
            [121, 0] => Some(HeroSpell::Blizzard),
            [122, 0] => Some(HeroSpell::DivineShield),
            [123, 0] => Some(HeroSpell::DivineShieldOff),
            [124, 0] => Some(HeroSpell::HolyLight),
            [125, 0] => Some(HeroSpell::MassTeleport),
            [126, 0] => Some(HeroSpell::Resurrection),
            [127, 0] => Some(HeroSpell::StormBolt),
            [128, 0] => Some(HeroSpell::ThunderClap),
            [129, 0] => Some(HeroSpell::SummonWaterElemental),
            [6, 2] => Some(HeroSpell::Banish),
            [7, 2] => Some(HeroSpell::SiphonMana),
            [8, 2] => Some(HeroSpell::FlameStrike),
            [9, 2] => Some(HeroSpell::SummonPhoenix),
            // OR
            [151, 0] => Some(HeroSpell::ChainLightning),
            [153, 0] => Some(HeroSpell::EarthQuake),
            [154, 0] => Some(HeroSpell::FarSight),
            [155, 0] => Some(HeroSpell::MirrorImage),
            [157, 0] => Some(HeroSpell::ShockWave),
            [158, 0] => Some(HeroSpell::FeralSpirit),
            [159, 0] => Some(HeroSpell::WarStomp),
            [160, 0] => Some(HeroSpell::Bladestorm),
            [161, 0] => Some(HeroSpell::WindWalk),
            [21, 2] => Some(HeroSpell::HealingWave),
            [22, 2] => Some(HeroSpell::Hex),
            [23, 2] => Some(HeroSpell::BigBadVoodoo),
            [24, 2] => Some(HeroSpell::SerpentWard),
            // NE
            [203, 0] => Some(HeroSpell::EntanglingRoots),
            [205, 0] => Some(HeroSpell::SearingArrows),
            [206, 0] => Some(HeroSpell::EnableAutoSearingArrows),
            [207, 0] => Some(HeroSpell::DisableAutoSearingArrows),
            [208, 0] => Some(HeroSpell::ForceOfNature),
            [209, 0] => Some(HeroSpell::ImmolationOn),
            [210, 0] => Some(HeroSpell::ImmolationOff),
            [211, 0] => Some(HeroSpell::ManaBurn),
            [212, 0] => Some(HeroSpell::Metamorphosis),
            [213, 0] => Some(HeroSpell::Scout),
            [215, 0] => Some(HeroSpell::Starfall),
            [216, 0] => Some(HeroSpell::Tranquility),
            // UD
            [249, 0] => Some(HeroSpell::AnimateDead),
            [250, 0] => Some(HeroSpell::LocustSwarm),
            [251, 0] => Some(HeroSpell::DarkRitual),
            [253, 0] => Some(HeroSpell::DeathAndDecay),
            [254, 0] => Some(HeroSpell::DeathCoil),
            [255, 0] => Some(HeroSpell::DeathPact),
            [0, 1] => Some(HeroSpell::Inferno),
            [1, 1] => Some(HeroSpell::FrostArmor),
            [2, 1] => Some(HeroSpell::FrostNova),
            [3, 1] => Some(HeroSpell::Sleep),
            [234, 1] => Some(HeroSpell::EnableAutoFrostArmor),
            [235, 1] => Some(HeroSpell::DisableAutoFrostArmor),

            // Neutral
            [14, 1] => Some(HeroSpell::RainOfFire),

            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnitSpell {
    ReturnResources,
    Mine,
    RevealArea,
    Repair,
    EnableAutoRepair,
    DisableAutoRepair,
    ReviveHero(usize),
    Kaboom,
    EnableAutoKaboom,
    DisableAutoKaboom,
    LoadUnit,
    UnloadUnit,
    UnloadAllUnits,
    AllWispsExitMine,
    EnableAutoLoadCorpses,
    DisableAutoLoadCorpses,
    LoadCorpses,
    UnloadCorpses,
    EnableDefend,
    DisableDefend,
    DispelMagic,
    Flare,
    Heal,
    EnableAutoHeal,
    DisableAutoHeal,
    InnerFire,
    EnableAutoInnerFire,
    DisableAutoInnerFire,
    Invisibility,
    CallToArmsPeasant,
    ReturnToWorkPeasant,
    Polymorph,
    Slow,
    EnableAutoSlow,
    DisableAutoSlow,
    CallToArmsTownHall,
    ReturnToWorkTownHall,
    BattleStations, // Orc burrow => into combat
    StandDown,      // Orc burrow => back to work
    Berserk,
    Bloodlust,
    EnableAutoBloodlust,
    DisableAutoBloodlust,
    Devour,
    SentryWard,
    Ensnare,
    HealingWard,
    LightningShield,
    Purge,
    StasisTrap,
    Shadowmeld,
    AbolishMagic,
    EnableAutoAbolishMagic,
    DisableAutoAbolishMagic,
    BearForm,
    DisableBearForm,
    PickUpArcher,
    MountHippogryph,
    Cyclone,
    Detonate,
    EatTree,
    EntangleGoldMine,
    FearyFire,
    EnableAutoFearyFire,
    DisableAutoFearyFire,
    CrowForm,
    DisableCrowForm,
    ReplenishManaAndLife,
    EnableAutoReplenishManaAndLife,
    DisableAutoReplenishManaAndLife,
    Rejuvenation,
    Renew, // N-E repair
    EnableAutoRenew,
    DisableAutoRenew,
    Roar,
    Root,
    Uproot,
    Sentinel,
    AntiMagicShell,
    Cannibalize,
    Cripple,
    Curse,
    EnableAutoCurse,
    DisableAutoCurse,
    Possession,
    RaiseDead,
    EnableAutoRaiseDead,
    DisableAutoRaiseDead,
    SacrificeAcolyte,
    Restore,
    EnableAutoRestore,
    DisableAutoRestore,
    SacrificePit,
    StoneForm,
    ExitStoneForm,
    UnholyFrenzy,
    Unsummon,
    Web,
    EnableAutoWeb,
    DisableAutoWeb,
    AnimateDead, // Satyre Hellcaller
    StormBolt,   // Golems
    ThunderClap, // Golems
    ReviveHeroFromTavern(usize),
    Cloud,
    ControlMagic,
    Shackles,
    SpellSteal,
    EnableAutoSpellSteal,
    DisableAutoSpellSteal,
    AncestralSpirit,
    ExitEtherealForm,
    EtherealForm,
    SpiritLink,
    UnstableConcoction,
    DecoupleArcher,
    WarClub,
    ManaFlare,
    PhaseShift,
    EnableAutoPhaseShift,
    DisableAutoPhaseShift,
    Taunt,
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
            [50, 0] => Some(UnitSpell::Mine),
            [55, 0] => Some(UnitSpell::RevealArea),
            [56, 0] => Some(UnitSpell::Repair),
            [57, 0] => Some(UnitSpell::EnableAutoRepair),
            [58, 0] => Some(UnitSpell::DisableAutoRepair),
            [59, 0] => Some(UnitSpell::ReviveHero(1)),
            [60, 0] => Some(UnitSpell::ReviveHero(2)),
            [61, 0] => Some(UnitSpell::ReviveHero(3)),
            [62, 0] => Some(UnitSpell::ReviveHero(4)),
            [63, 0] => Some(UnitSpell::ReviveHero(5)),
            [72, 0] => Some(UnitSpell::Kaboom),
            [73, 0] => Some(UnitSpell::EnableAutoKaboom),
            [74, 0] => Some(UnitSpell::DisableAutoKaboom),
            [78, 0] => Some(UnitSpell::LoadUnit),
            [79, 0] => Some(UnitSpell::UnloadUnit),
            [80, 0] => Some(UnitSpell::UnloadAllUnits),
            [81, 0] => Some(UnitSpell::AllWispsExitMine),
            [83, 0] => Some(UnitSpell::EnableAutoLoadCorpses),
            [84, 0] => Some(UnitSpell::DisableAutoLoadCorpses),
            [85, 0] => Some(UnitSpell::LoadCorpses),
            [86, 0] => Some(UnitSpell::UnloadCorpses),
            [87, 0] => Some(UnitSpell::EnableDefend),
            [88, 0] => Some(UnitSpell::DisableDefend),
            [89, 0] => Some(UnitSpell::AbolishMagic),
            [92, 0] => Some(UnitSpell::Flare),
            [95, 0] => Some(UnitSpell::Heal),
            [96, 0] => Some(UnitSpell::EnableAutoHeal),
            [97, 0] => Some(UnitSpell::DisableAutoHeal),
            [98, 0] => Some(UnitSpell::InnerFire),
            [99, 0] => Some(UnitSpell::EnableAutoInnerFire),
            [100, 0] => Some(UnitSpell::DisableAutoInnerFire),
            [101, 0] => Some(UnitSpell::Invisibility),
            [104, 0] => Some(UnitSpell::CallToArmsPeasant),
            [105, 0] => Some(UnitSpell::ReturnToWorkPeasant),
            [106, 0] => Some(UnitSpell::Polymorph),
            [107, 0] => Some(UnitSpell::Slow),
            [108, 0] => Some(UnitSpell::EnableAutoSlow),
            [109, 0] => Some(UnitSpell::DisableAutoSlow),
            [114, 0] => Some(UnitSpell::CallToArmsTownHall),
            [115, 0] => Some(UnitSpell::ReturnToWorkTownHall),
            [131, 0] => Some(UnitSpell::BattleStations),
            [132, 0] => Some(UnitSpell::Berserk),
            [133, 0] => Some(UnitSpell::Bloodlust),
            [134, 0] => Some(UnitSpell::EnableAutoBloodlust),
            [135, 0] => Some(UnitSpell::DisableAutoBloodlust),
            [136, 0] => Some(UnitSpell::Devour),
            [137, 0] => Some(UnitSpell::SentryWard),
            [138, 0] => Some(UnitSpell::Ensnare),
            [141, 0] => Some(UnitSpell::HealingWard),
            [142, 0] => Some(UnitSpell::LightningShield),
            [143, 0] => Some(UnitSpell::Purge),
            [145, 0] => Some(UnitSpell::StandDown),
            [146, 0] => Some(UnitSpell::StasisTrap),
            [163, 0] => Some(UnitSpell::Shadowmeld),
            [164, 0] => Some(UnitSpell::AbolishMagic),
            [165, 0] => Some(UnitSpell::EnableAutoAbolishMagic),
            [166, 0] => Some(UnitSpell::DisableAutoAbolishMagic),
            [170, 0] => Some(UnitSpell::BearForm),
            [171, 0] => Some(UnitSpell::DisableBearForm),
            [174, 0] => Some(UnitSpell::PickUpArcher),
            [175, 0] => Some(UnitSpell::MountHippogryph),
            [176, 0] => Some(UnitSpell::Cyclone),
            [177, 0] => Some(UnitSpell::Detonate),
            [178, 0] => Some(UnitSpell::EatTree),
            [179, 0] => Some(UnitSpell::EntangleGoldMine),
            [181, 0] => Some(UnitSpell::FearyFire),
            [182, 0] => Some(UnitSpell::EnableAutoFearyFire),
            [183, 0] => Some(UnitSpell::DisableAutoFearyFire),
            [187, 0] => Some(UnitSpell::CrowForm),
            [188, 0] => Some(UnitSpell::DisableCrowForm),
            [189, 0] => Some(UnitSpell::ReplenishManaAndLife),
            [190, 0] => Some(UnitSpell::EnableAutoReplenishManaAndLife),
            [191, 0] => Some(UnitSpell::DisableAutoReplenishManaAndLife),
            [192, 0] => Some(UnitSpell::Rejuvenation),
            [193, 0] => Some(UnitSpell::Renew),
            [194, 0] => Some(UnitSpell::EnableAutoRenew),
            [195, 0] => Some(UnitSpell::DisableAutoRenew),
            [196, 0] => Some(UnitSpell::Roar),
            [197, 0] => Some(UnitSpell::Root),
            [198, 0] => Some(UnitSpell::Uproot),
            [214, 0] => Some(UnitSpell::Sentinel),
            [218, 0] => Some(UnitSpell::AntiMagicShell),
            [220, 0] => Some(UnitSpell::Cannibalize),
            [221, 0] => Some(UnitSpell::Cripple),
            [222, 0] => Some(UnitSpell::Curse),
            [223, 0] => Some(UnitSpell::EnableAutoCurse),
            [224, 0] => Some(UnitSpell::DisableAutoCurse),
            [228, 0] => Some(UnitSpell::Possession),
            [230, 0] => Some(UnitSpell::EnableAutoRaiseDead),
            [231, 0] => Some(UnitSpell::DisableAutoRaiseDead),
            [232, 0] => Some(UnitSpell::RaiseDead),
            [233, 0] => Some(UnitSpell::SacrificeAcolyte),
            [234, 0] => Some(UnitSpell::Restore),
            [235, 0] => Some(UnitSpell::EnableAutoRestore),
            [236, 0] => Some(UnitSpell::DisableAutoRestore),
            [237, 0] => Some(UnitSpell::SacrificePit),
            [238, 0] => Some(UnitSpell::StoneForm),
            [239, 0] => Some(UnitSpell::ExitStoneForm),
            [241, 0] => Some(UnitSpell::UnholyFrenzy),
            [242, 0] => Some(UnitSpell::Unsummon),
            [243, 0] => Some(UnitSpell::Web),
            [244, 0] => Some(UnitSpell::EnableAutoWeb),
            [245, 0] => Some(UnitSpell::DisableAutoWeb),
            [22, 1] => Some(UnitSpell::AnimateDead),
            [23, 1] => Some(UnitSpell::Devour),         // Lizards
            [24, 1] => Some(UnitSpell::Heal),           // Forest troll SP
            [25, 1] => Some(UnitSpell::EnableAutoHeal), // Forest troll SP
            [26, 1] => Some(UnitSpell::DisableAutoHeal), // Forest troll SP
            [28, 1] => Some(UnitSpell::StormBolt),
            [29, 1] => Some(UnitSpell::ThunderClap),
            [46, 1] => Some(UnitSpell::RevealArea), // Human arcane tower
            [238, 1] => Some(UnitSpell::ReviveHeroFromTavern(0)),
            [239, 1] => Some(UnitSpell::ReviveHeroFromTavern(1)),
            [240, 1] => Some(UnitSpell::ReviveHeroFromTavern(2)),
            [241, 1] => Some(UnitSpell::ReviveHeroFromTavern(3)),
            [242, 1] => Some(UnitSpell::ReviveHeroFromTavern(4)),
            [249, 1] => Some(UnitSpell::Cloud),
            [250, 1] => Some(UnitSpell::ControlMagic),
            [0, 2] => Some(UnitSpell::Shackles),
            [3, 2] => Some(UnitSpell::SpellSteal),
            [4, 2] => Some(UnitSpell::EnableAutoSpellSteal),
            [5, 2] => Some(UnitSpell::DisableAutoSpellSteal),
            [10, 2] => Some(UnitSpell::AncestralSpirit),
            [13, 2] => Some(UnitSpell::ExitEtherealForm),
            [14, 2] => Some(UnitSpell::EtherealForm),
            [19, 2] => Some(UnitSpell::SpiritLink),
            [20, 2] => Some(UnitSpell::UnstableConcoction),
            [28, 2] => Some(UnitSpell::PickUpArcher),
            [29, 2] => Some(UnitSpell::DecoupleArcher),
            [31, 2] => Some(UnitSpell::WarClub),
            [32, 2] | [33, 2] => Some(UnitSpell::ManaFlare),
            [34, 2] => Some(UnitSpell::PhaseShift),
            [35, 2] => Some(UnitSpell::EnableAutoPhaseShift),
            [36, 2] => Some(UnitSpell::DisableAutoPhaseShift),
            [40, 2] => Some(UnitSpell::Taunt),

            // TODO: stopped here
            _ => None,
        }
    }
}

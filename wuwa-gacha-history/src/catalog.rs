//! Resonator (character) and Weapon catalogs for Wuthering Waves (global / EN names).
//!
//! Sources:
//! - Wuthering Waves global website (wutheringwaves.kurogames.com)
//! - game8.co, wuthering.gg community databases
//!
//! Coverage: up to Version 3.2 (April 2026).
//!
//! Standard 5★ Resonators are Calcharo, Encore, Jianxin, Jiyan, Lingyang, Verina
//! (plus the story-only Rover variants). Every other 5★ is Limited.
//! Standard 5★ weapons are the 5 Winter Brume weapons (launch) and the 5 Synth
//! Armament weapons (v3.0) — 10 total. Every other 5★ is Limited.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Resonator {
    Limited5Star(Limited5StarResonator),
    Standard5Star(Standard5StarResonator),
    FourStar(FourStarResonator),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Limited5StarResonator {
    Aemeath,
    Augusta,
    Brant,
    Camellya,
    Cantarella,
    Carlotta,
    Cartethyia,
    Changli,
    Chisa,
    Ciaccona,
    Denia,
    Galbrena,
    Hiyuki,
    Iuno,
    Jinhsi,
    Lupa,
    LuukHerssen,
    Lynae,
    Mornye,
    Phoebe,
    Phrolova,
    Qiuyuan,
    Roccia,
    Shorekeeper,
    Sigrika,
    XiangliYao,
    Yinlin,
    Zani,
    Zhezhi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Standard5StarResonator {
    Calcharo,
    Encore,
    Jianxin,
    Jiyan,
    Lingyang,
    Verina,
    RoverFemaleAero,
    RoverFemaleHavoc,
    RoverFemaleSpectro,
    RoverMaleAero,
    RoverMaleHavoc,
    RoverMaleSpectro,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FourStarResonator {
    Aalto,
    Baizhi,
    Buling,
    Chixia,
    Danjin,
    Lumi,
    Mortefi,
    Sanhua,
    Taoqi,
    Yangyang,
    Youhu,
    Yuanwu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Weapon {
    Limited5Star(Limited5StarWeapon),
    Standard5Star(Standard5StarWeapon),
    FourStar(FourStarWeapon),
    ThreeStar(ThreeStarWeapon),
    TwoStar(TwoStarWeapon),
    OneStar(OneStarWeapon),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Limited5StarWeapon {
    AgesOfHarvest,        // Broadblade
    BlazingBrilliance,    // Sword
    BlazingJustice,       // Gauntlet
    BloodpactsPledge,     // Sword
    DaybreakersSpine,     // Gauntlet
    DefiersThorn,         // Sword
    EmeraldSentence,      // Sword
    EverbrightPolestar,   // Sword
    Kumokiri,             // Broadblade
    LetheanElegy,         // Rectifier
    LuminousHymn,         // Rectifier
    LuxAndUmbra,          // Pistol
    MoongazersSigil,      // Gauntlet
    RedSpring,            // Sword
    RimeDrapedSprouts,    // Rectifier
    SolswornCiphers,      // Gauntlet
    SpectrumBlaster,      // Pistol
    StarfieldCalibrator,  // Broadblade
    StellarSymphony,      // Rectifier
    Stringmaster,         // Rectifier
    TheLastDance,         // Pistol
    ThunderflareDominion, // Broadblade
    Tragicomedy,          // Gauntlet
    UnflickeringValor,    // Sword
    VerdantSummit,        // Broadblade
    VeritysHandle,        // Gauntlet
    WhispersOfSirens,     // Rectifier
    WildfireMark,         // Broadblade
    WoodlandAria,         // Pistol
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Standard5StarWeapon {
    // Winter Brume (launch, 1.0)
    LustrousRazor,    // Broadblade
    EmeraldOfGenesis, // Sword
    StaticMist,       // Pistol
    AbyssSurges,      // Gauntlet
    CosmicRipples,    // Rectifier
    // Synth Armament (3.0)
    RadianceCleaver,   // Broadblade
    LaserShearer,      // Sword
    PhasicHomogenizer, // Pistol
    PulsationBracer,   // Gauntlet
    BosonAstrolabe,    // Rectifier
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FourStarWeapon {
    // Broadblade
    Autumntrace,
    AureateZenith,
    Broadblade41,
    DauntlessEvernight,
    Discord,
    HeliosCleaver,
    MeditationsOnMercy,
    WaningRedshift,
    // Sword
    CommandoOfConviction,
    EndlessCollapse,
    FablesOfWisdom,
    FeatherEdge,
    Lumingloss,
    LunarCutter,
    Overture,
    SomnoireAnchor,
    Sword18,
    // Pistol
    Cadenza,
    Novaburst,
    Pistols26,
    RelativisticJet,
    RomanceInFarewell,
    SolarFlame,
    Thunderbolt,
    UndyingFlame,
    // Gauntlet
    AetherStrike,
    AmityAccord,
    CelestialSpiral,
    Gauntlets21D,
    HollowMirage,
    LegendOfDrunkenHero,
    Marcato,
    Stonard,
    // Rectifier
    Augment,
    CallOfTheAbyss,
    CometFlare,
    FusionAccretion,
    JinzhouKeeper,
    OceansGift,
    RadiantDawn,
    Rectifier25,
    Variation,
    WaltzInMasquerade,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThreeStarWeapon {
    // Broadblade
    BeguilingMelody,
    BroadbladeOfNight,
    BroadbladeOfVoyager,
    GuardianBroadblade,
    OriginiteTypeI,
    // Sword
    GuardianSword,
    OriginiteTypeII,
    SwordOfNight,
    SwordOfVoyager,
    // Pistol
    GuardianPistols,
    OriginiteTypeIII,
    PistolsOfNight,
    PistolsOfVoyager,
    // Gauntlet
    GauntletsOfNight,
    GauntletsOfVoyager,
    GuardianGauntlets,
    OriginiteTypeIV,
    // Rectifier
    GuardianRectifier,
    OriginiteTypeV,
    RectifierOfNight,
    RectifierOfVoyager,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TwoStarWeapon {
    TyroBroadblade,
    TyroSword,
    TyroPistols,
    TyroGauntlets,
    TyroRectifier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OneStarWeapon {
    TrainingBroadblade,
    TrainingSword,
    TrainingPistols,
    TrainingGauntlets,
    TrainingRectifier,
}

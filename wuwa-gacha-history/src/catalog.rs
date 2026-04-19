//! Resonator (character) and Weapon catalogs with CN/EN names and asset paths.
//!
//! Counts mirror `public/wiki-art/`: 53 Resonators, 113 Weapons (v3.2, 2026-04).
//!
//! Standard 5★ Resonators: Calcharo, Encore, Jianxin, Jiyan, Lingyang, Verina
//! (+ 6 Rover variants). Every other 5★ is Limited.
//!
//! Standard 5★ Weapons (10 total): Winter Brume (launch) + Synth Armament (3.0).
//! Every other 5★ is Limited signature. 1★ = Training series, 2★ = Tyro series.
//!
//! Weapon CN↔EN mappings for signature weapons are best-effort where the
//! mapping is not publicly confirmed; series weapons are mechanical and exact.

use std::path::PathBuf;

const CHARACTER_ASSET_DIR: &str = "wiki-art/characters";
const WEAPON_ASSET_DIR: &str = "wiki-art/weapons";

fn asset_path(dir: &str, zh: &str) -> PathBuf {
    PathBuf::from(dir).join(format!("{zh}.png"))
}

// -------------------------- Resonators --------------------------

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

impl Resonator {
    pub fn zh(&self) -> String {
        match self {
            Self::Limited5Star(v) => v.zh(),
            Self::Standard5Star(v) => v.zh(),
            Self::FourStar(v) => v.zh(),
        }
    }
    pub fn en(&self) -> String {
        match self {
            Self::Limited5Star(v) => v.en(),
            Self::Standard5Star(v) => v.en(),
            Self::FourStar(v) => v.en(),
        }
    }
    pub fn asset(&self) -> PathBuf {
        match self {
            Self::Limited5Star(v) => v.asset(),
            Self::Standard5Star(v) => v.asset(),
            Self::FourStar(v) => v.asset(),
        }
    }
}

impl Limited5StarResonator {
    fn data(&self) -> (&'static str, &'static str) {
        use Limited5StarResonator::*;
        match self {
            Aemeath => ("爱弥斯", "Aemeath"),
            Augusta => ("奥古斯塔", "Augusta"),
            Brant => ("布兰特", "Brant"),
            Camellya => ("椿", "Camellya"),
            Cantarella => ("坎特蕾拉", "Cantarella"),
            Carlotta => ("珂莱塔", "Carlotta"),
            Cartethyia => ("卡提希娅", "Cartethyia"),
            Changli => ("长离", "Changli"),
            Chisa => ("千咲", "Chisa"),
            Ciaccona => ("夏空", "Ciaccona"),
            Denia => ("达妮娅", "Denia"),
            Galbrena => ("嘉贝莉娜", "Galbrena"),
            Hiyuki => ("绯雪", "Hiyuki"),
            Iuno => ("尤诺", "Iuno"),
            Jinhsi => ("今汐", "Jinhsi"),
            Lupa => ("露帕", "Lupa"),
            LuukHerssen => ("陆·赫斯", "Luuk Herssen"),
            Lynae => ("琳奈", "Lynae"),
            Mornye => ("莫宁", "Mornye"),
            Phoebe => ("菲比", "Phoebe"),
            Phrolova => ("弗洛洛", "Phrolova"),
            Qiuyuan => ("仇远", "Qiuyuan"),
            Roccia => ("洛可可", "Roccia"),
            Shorekeeper => ("守岸人", "Shorekeeper"),
            Sigrika => ("西格莉卡", "Sigrika"),
            XiangliYao => ("相里要", "Xiangli Yao"),
            Yinlin => ("吟霖", "Yinlin"),
            Zani => ("赞妮", "Zani"),
            Zhezhi => ("折枝", "Zhezhi"),
        }
    }
    pub fn zh(&self) -> String {
        self.data().0.to_string()
    }
    pub fn en(&self) -> String {
        self.data().1.to_string()
    }
    pub fn asset(&self) -> PathBuf {
        asset_path(CHARACTER_ASSET_DIR, self.data().0)
    }
}

impl Standard5StarResonator {
    fn data(&self) -> (&'static str, &'static str) {
        use Standard5StarResonator::*;
        match self {
            Calcharo => ("卡卡罗", "Calcharo"),
            Encore => ("安可", "Encore"),
            Jianxin => ("鉴心", "Jianxin"),
            Jiyan => ("忌炎", "Jiyan"),
            Lingyang => ("凌阳", "Lingyang"),
            Verina => ("维里奈", "Verina"),
            RoverFemaleAero => ("漂泊者-女-气动", "Rover (Female, Aero)"),
            RoverFemaleHavoc => ("漂泊者-女-湮灭", "Rover (Female, Havoc)"),
            RoverFemaleSpectro => ("漂泊者-女-衍射", "Rover (Female, Spectro)"),
            RoverMaleAero => ("漂泊者-男-气动", "Rover (Male, Aero)"),
            RoverMaleHavoc => ("漂泊者-男-湮灭", "Rover (Male, Havoc)"),
            RoverMaleSpectro => ("漂泊者-男-衍射", "Rover (Male, Spectro)"),
        }
    }
    pub fn zh(&self) -> String {
        self.data().0.to_string()
    }
    pub fn en(&self) -> String {
        self.data().1.to_string()
    }
    pub fn asset(&self) -> PathBuf {
        asset_path(CHARACTER_ASSET_DIR, self.data().0)
    }
}

impl FourStarResonator {
    fn data(&self) -> (&'static str, &'static str) {
        use FourStarResonator::*;
        match self {
            Aalto => ("秋水", "Aalto"),
            Baizhi => ("白芷", "Baizhi"),
            Buling => ("卜灵", "Buling"),
            Chixia => ("炽霞", "Chixia"),
            Danjin => ("丹瑾", "Danjin"),
            Lumi => ("灯灯", "Lumi"),
            Mortefi => ("莫特斐", "Mortefi"),
            Sanhua => ("散华", "Sanhua"),
            Taoqi => ("桃祈", "Taoqi"),
            Yangyang => ("秧秧", "Yangyang"),
            Youhu => ("釉瑚", "Youhu"),
            Yuanwu => ("渊武", "Yuanwu"),
        }
    }
    pub fn zh(&self) -> String {
        self.data().0.to_string()
    }
    pub fn en(&self) -> String {
        self.data().1.to_string()
    }
    pub fn asset(&self) -> PathBuf {
        asset_path(CHARACTER_ASSET_DIR, self.data().0)
    }
}

// -------------------------- Weapons --------------------------

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
    // Broadblades
    AgesOfHarvest,
    Kumokiri,
    StarfieldCalibrator,
    ThunderflareDominion,
    VerdantSummit,
    WildfireMark,
    // Swords
    BlazingBrilliance,
    BloodpactsPledge,
    DefiersThorn,
    EmeraldSentence,
    EverbrightPolestar,
    RedSpring,
    UnflickeringValor,
    // Pistols
    LuxAndUmbra,
    SpectrumBlaster,
    TheLastDance,
    WoodlandAria,
    // Gauntlets
    BlazingJustice,
    DaybreakersSpine,
    MoongazersSigil,
    SolswornCiphers,
    Tragicomedy,
    VeritysHandle,
    // Rectifiers
    LetheanElegy,
    LuminousHymn,
    RimeDrapedSprouts,
    StellarSymphony,
    Stringmaster,
    WhispersOfSirens,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Standard5StarWeapon {
    // Winter Brume (launch, 1.0)
    LustrousRazor,
    EmeraldOfGenesis,
    StaticMist,
    AbyssSurges,
    CosmicRipples,
    // Synth Armament (3.0)
    RadianceCleaver,
    LaserShearer,
    PhasicHomogenizer,
    PulsationBracer,
    BosonAstrolabe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FourStarWeapon {
    // Broadblades
    Autumntrace,
    AureateZenith,
    Broadblade41,
    DauntlessEvernight,
    Discord,
    HeliosCleaver,
    MeditationsOnMercy,
    WaningRedshift,
    // Swords
    CommandoOfConviction,
    EndlessCollapse,
    FablesOfWisdom,
    FeatherEdge,
    Lumingloss,
    LunarCutter,
    Overture,
    SomnoireAnchor,
    Sword18,
    // Pistols
    Cadenza,
    Novaburst,
    Pistols26,
    RelativisticJet,
    RomanceInFarewell,
    SolarFlame,
    Thunderbolt,
    UndyingFlame,
    // Gauntlets
    AetherStrike,
    AmityAccord,
    CelestialSpiral,
    Gauntlets21D,
    HollowMirage,
    LegendOfDrunkenHero,
    Marcato,
    Stonard,
    // Rectifiers
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
    // Broadblades
    BeguilingMelody,
    BroadbladeOfNight,
    BroadbladeOfVoyager,
    GuardianBroadblade,
    OriginiteTypeI,
    // Swords
    GuardianSword,
    OriginiteTypeII,
    SwordOfNight,
    SwordOfVoyager,
    // Pistols
    GuardianPistols,
    OriginiteTypeIII,
    PistolsOfNight,
    PistolsOfVoyager,
    // Gauntlets
    GauntletsOfNight,
    GauntletsOfVoyager,
    GuardianGauntlets,
    OriginiteTypeIV,
    // Rectifiers
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

impl Weapon {
    pub fn zh(&self) -> String {
        match self {
            Self::Limited5Star(v) => v.zh(),
            Self::Standard5Star(v) => v.zh(),
            Self::FourStar(v) => v.zh(),
            Self::ThreeStar(v) => v.zh(),
            Self::TwoStar(v) => v.zh(),
            Self::OneStar(v) => v.zh(),
        }
    }
    pub fn en(&self) -> String {
        match self {
            Self::Limited5Star(v) => v.en(),
            Self::Standard5Star(v) => v.en(),
            Self::FourStar(v) => v.en(),
            Self::ThreeStar(v) => v.en(),
            Self::TwoStar(v) => v.en(),
            Self::OneStar(v) => v.en(),
        }
    }
    pub fn asset(&self) -> PathBuf {
        match self {
            Self::Limited5Star(v) => v.asset(),
            Self::Standard5Star(v) => v.asset(),
            Self::FourStar(v) => v.asset(),
            Self::ThreeStar(v) => v.asset(),
            Self::TwoStar(v) => v.asset(),
            Self::OneStar(v) => v.asset(),
        }
    }
}

impl Limited5StarWeapon {
    fn data(&self) -> (&'static str, &'static str) {
        use Limited5StarWeapon::*;
        match self {
            // Broadblades
            AgesOfHarvest => ("时和岁稔", "Ages of Harvest"),
            Kumokiri => ("昙切", "Kumokiri"),
            StarfieldCalibrator => ("宙算仪轨", "Starfield Calibrator"),
            ThunderflareDominion => ("驭冕铸雷之权", "Thunderflare Dominion"),
            VerdantSummit => ("浩境粼光", "Verdant Summit"),
            WildfireMark => ("焰痕", "Wildfire Mark"),
            // Swords
            BlazingBrilliance => ("赫奕流明", "Blazing Brilliance"),
            BloodpactsPledge => ("血誓盟约", "Bloodpact's Pledge"),
            DefiersThorn => ("裁竹", "Defier's Thorn"),
            EmeraldSentence => ("不灭航路", "Emerald Sentence"),
            EverbrightPolestar => ("永远的启明星", "Everbright Polestar"),
            RedSpring => ("裁春", "Red Spring"),
            UnflickeringValor => ("不屈命定之冠", "Unflickering Valor"),
            // Pistols
            LuxAndUmbra => ("光影双生", "Lux & Umbra"),
            SpectrumBlaster => ("溢彩荧辉", "Spectrum Blaster"),
            TheLastDance => ("死与舞", "The Last Dance"),
            WoodlandAria => ("林间的咏叹调", "Woodland Aria"),
            // Gauntlets
            BlazingJustice => ("脉冲协臂", "Blazing Justice"),
            DaybreakersSpine => ("白昼之脊", "Daybreaker's Spine"),
            MoongazersSigil => ("金掌", "Moongazer's Sigil"),
            SolswornCiphers => ("万物持存的注释", "Solsworn Ciphers"),
            Tragicomedy => ("悲喜剧", "Tragicomedy"),
            VeritysHandle => ("诸方玄枢", "Verity's Handle"),
            // Rectifiers
            LetheanElegy => ("幽冥的忘忧章", "Lethean Elegy"),
            LuminousHymn => ("和光回唱", "Luminous Hymn"),
            RimeDrapedSprouts => ("琼枝冰绡", "Rime-Draped Sprouts"),
            StellarSymphony => ("星序协响", "Stellar Symphony"),
            Stringmaster => ("掣傀之手", "Stringmaster"),
            WhispersOfSirens => ("海的呢喃", "Whispers of Sirens"),
        }
    }
    pub fn zh(&self) -> String {
        self.data().0.to_string()
    }
    pub fn en(&self) -> String {
        self.data().1.to_string()
    }
    pub fn asset(&self) -> PathBuf {
        asset_path(WEAPON_ASSET_DIR, self.data().0)
    }
}

impl Standard5StarWeapon {
    fn data(&self) -> (&'static str, &'static str) {
        use Standard5StarWeapon::*;
        match self {
            LustrousRazor => ("苍鳞千嶂", "Lustrous Razor"),
            EmeraldOfGenesis => ("千古洑流", "Emerald of Genesis"),
            StaticMist => ("停驻之烟", "Static Mist"),
            AbyssSurges => ("擎渊怒涛", "Abyss Surges"),
            CosmicRipples => ("漪澜浮录", "Cosmic Ripples"),
            RadianceCleaver => ("源能机锋", "Radiance Cleaver"),
            LaserShearer => ("镭射切变", "Laser Shearer"),
            PhasicHomogenizer => ("相位涟漪", "Phasic Homogenizer"),
            PulsationBracer => ("尘云旋臂", "Pulsation Bracer"),
            BosonAstrolabe => ("玻色星仪", "Boson Astrolabe"),
        }
    }
    pub fn zh(&self) -> String {
        self.data().0.to_string()
    }
    pub fn en(&self) -> String {
        self.data().1.to_string()
    }
    pub fn asset(&self) -> PathBuf {
        asset_path(WEAPON_ASSET_DIR, self.data().0)
    }
}

impl FourStarWeapon {
    fn data(&self) -> (&'static str, &'static str) {
        use FourStarWeapon::*;
        match self {
            // Broadblades
            Autumntrace => ("纹秋", "Autumntrace"),
            AureateZenith => ("金穹", "Aureate Zenith"),
            Broadblade41 => ("重破刃-41型", "Broadblade#41"),
            DauntlessEvernight => ("永夜长明", "Dauntless Evernight"),
            Discord => ("异响空灵", "Discord"),
            HeliosCleaver => ("昭日译注", "Helios Cleaver"),
            MeditationsOnMercy => ("容赦的沉思录", "Meditations on Mercy"),
            WaningRedshift => ("骇行", "Waning Redshift"),
            // Swords
            CommandoOfConviction => ("不归孤军", "Commando of Conviction"),
            EndlessCollapse => ("永续坍缩", "Endless Collapse"),
            FablesOfWisdom => ("风流的寓言诗", "Fables of Wisdom"),
            FeatherEdge => ("翼锋", "Feather Edge"),
            Lumingloss => ("清音", "Lumingloss"),
            LunarCutter => ("飞逝", "Lunar Cutter"),
            Overture => ("行进序曲", "Overture"),
            SomnoireAnchor => ("心之锚", "Somnoire Anchor"),
            Sword18 => ("瞬斩刀-18型", "Sword#18"),
            // Pistols
            Cadenza => ("华彩乐段", "Cadenza"),
            Novaburst => ("飞景", "Novaburst"),
            Pistols26 => ("穿击枪-26型", "Pistols#26"),
            RelativisticJet => ("悖论喷流", "Relativistic Jet"),
            RomanceInFarewell => ("叙别的罗曼史", "Romance in Farewell"),
            SolarFlame => ("阳焰", "Solar Flame"),
            Thunderbolt => ("奔雷", "Thunderbolt"),
            UndyingFlame => ("无眠烈火", "Undying Flame"),
            // Gauntlets
            AetherStrike => ("凌空", "Aether Strike"),
            AmityAccord => ("东落", "Amity Accord"),
            CelestialSpiral => ("西升", "Celestial Spiral"),
            Gauntlets21D => ("钢影拳-21丁型", "Gauntlets#21D"),
            HollowMirage => ("凋亡频移", "Hollow Mirage"),
            LegendOfDrunkenHero => ("酩酊的英雄志", "Legend of Drunken Hero"),
            Marcato => ("异度", "Marcato"),
            Stonard => ("袍泽之固", "Stonard"),
            // Rectifiers
            Augment => ("呼啸重音", "Augment"),
            CallOfTheAbyss => ("渊海回声", "Call of the Abyss"),
            CometFlare => ("焰光裁定", "Comet Flare"),
            FusionAccretion => ("核熔星盘", "Fusion Accretion"),
            JinzhouKeeper => ("今州守望", "Jinzhou Keeper"),
            OceansGift => ("大海的馈赠", "Ocean's Gift"),
            RadiantDawn => ("曜光", "Radiant Dawn"),
            Rectifier25 => ("鸣动仪-25型", "Rectifier#25"),
            Variation => ("奇幻变奏", "Variation"),
            WaltzInMasquerade => ("虚饰的华尔兹", "Waltz in Masquerade"),
        }
    }
    pub fn zh(&self) -> String {
        self.data().0.to_string()
    }
    pub fn en(&self) -> String {
        self.data().1.to_string()
    }
    pub fn asset(&self) -> PathBuf {
        asset_path(WEAPON_ASSET_DIR, self.data().0)
    }
}

impl ThreeStarWeapon {
    fn data(&self) -> (&'static str, &'static str) {
        use ThreeStarWeapon::*;
        match self {
            // Broadblades
            BeguilingMelody => ("钧天正音", "Beguiling Melody"),
            BroadbladeOfNight => ("暗夜长刃·玄明", "Broadblade of Night"),
            BroadbladeOfVoyager => ("远行者长刃·辟路", "Broadblade of Voyager"),
            GuardianBroadblade => ("戍关长刃·定军", "Guardian Broadblade"),
            OriginiteTypeI => ("源能长刃·测壹", "Originite: Type I"),
            // Swords
            GuardianSword => ("戍关迅刀·镇海", "Guardian Sword"),
            OriginiteTypeII => ("源能迅刀·测贰", "Originite: Type II"),
            SwordOfNight => ("暗夜迅刀·黑闪", "Sword of Night"),
            SwordOfVoyager => ("远行者迅刀·旅迹", "Sword of Voyager"),
            // Pistols
            GuardianPistols => ("戍关佩枪·平云", "Guardian Pistols"),
            OriginiteTypeIII => ("源能佩枪·测叁", "Originite: Type III"),
            PistolsOfNight => ("暗夜佩枪·暗星", "Pistols of Night"),
            PistolsOfVoyager => ("远行者佩枪·洞察", "Pistols of Voyager"),
            // Gauntlets
            GauntletsOfNight => ("暗夜臂铠·夜芒", "Gauntlets of Night"),
            GauntletsOfVoyager => ("远行者臂铠·破障", "Gauntlets of Voyager"),
            GuardianGauntlets => ("戍关臂铠·拔山", "Guardian Gauntlets"),
            OriginiteTypeIV => ("源能臂铠·测肆", "Originite: Type IV"),
            // Rectifiers
            GuardianRectifier => ("戍关音感仪·留光", "Guardian Rectifier"),
            OriginiteTypeV => ("源能音感仪·测五", "Originite: Type V"),
            RectifierOfNight => ("暗夜矩阵·暝光", "Rectifier of Night"),
            RectifierOfVoyager => ("远行者矩阵·探幽", "Rectifier of Voyager"),
        }
    }
    pub fn zh(&self) -> String {
        self.data().0.to_string()
    }
    pub fn en(&self) -> String {
        self.data().1.to_string()
    }
    pub fn asset(&self) -> PathBuf {
        asset_path(WEAPON_ASSET_DIR, self.data().0)
    }
}

impl TwoStarWeapon {
    fn data(&self) -> (&'static str, &'static str) {
        use TwoStarWeapon::*;
        match self {
            TyroBroadblade => ("原初长刃·朴石", "Tyro Broadblade"),
            TyroSword => ("原初迅刀·鸣雨", "Tyro Sword"),
            TyroPistols => ("原初佩枪·穿林", "Tyro Pistols"),
            TyroGauntlets => ("原初臂铠·磐岩", "Tyro Gauntlets"),
            TyroRectifier => ("原初音感仪·听浪", "Tyro Rectifier"),
        }
    }
    pub fn zh(&self) -> String {
        self.data().0.to_string()
    }
    pub fn en(&self) -> String {
        self.data().1.to_string()
    }
    pub fn asset(&self) -> PathBuf {
        asset_path(WEAPON_ASSET_DIR, self.data().0)
    }
}

impl OneStarWeapon {
    fn data(&self) -> (&'static str, &'static str) {
        use OneStarWeapon::*;
        match self {
            TrainingBroadblade => ("教学长刃", "Training Broadblade"),
            TrainingSword => ("教学迅刀", "Training Sword"),
            TrainingPistols => ("教学佩枪", "Training Pistols"),
            TrainingGauntlets => ("教学臂铠", "Training Gauntlets"),
            TrainingRectifier => ("教学音感仪", "Training Rectifier"),
        }
    }
    pub fn zh(&self) -> String {
        self.data().0.to_string()
    }
    pub fn en(&self) -> String {
        self.data().1.to_string()
    }
    pub fn asset(&self) -> PathBuf {
        asset_path(WEAPON_ASSET_DIR, self.data().0)
    }
}

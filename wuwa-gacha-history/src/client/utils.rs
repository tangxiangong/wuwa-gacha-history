#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub enum CardPool {
    FeaturedResonatorConvene = 1,
    FeaturedWeaponConvene = 2,
    StandardResonatorConvene = 3,
    StandardWeaponConvene = 4,
    NoviceConvene = 5,
    BeginnerChoiceConvene = 6,
    GivebackCustomConvene = 7,
}

impl std::fmt::Display for CardPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FeaturedResonatorConvene => write!(f, "限定角色"),
            Self::FeaturedWeaponConvene => write!(f, "限定武器"),
            Self::StandardResonatorConvene => write!(f, "常驻角色"),
            Self::StandardWeaponConvene => write!(f, "常驻武器"),
            Self::NoviceConvene => write!(f, "新手唤取"),
            Self::BeginnerChoiceConvene => write!(f, "新手自选"),
            Self::GivebackCustomConvene => write!(f, "感恩自选"),
        }
    }
}

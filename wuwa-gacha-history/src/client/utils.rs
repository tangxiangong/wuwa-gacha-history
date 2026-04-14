const STANDARD_RESONATOR_CONVENE_ID: &str = "6994d9b2-88d3-4efa-b33e-4c7a297b5d0e";
const STANDARD_WEAPON_CONVENE_ID: &str = "2e6e7c2b-d925-42b8-9a2f-1a57c3b6d9e0";
const NOVICE_CONVENE_ID: &str = "e0fa20f7-8a2b-4c5b-9de8-8e5a3c2e4d7f";
const BEGINNER_CHOICE_CONVENE_ID: &str = "d3aa37e3-a8b4-4d5c-8a1e-5e7b9c2d1f3a";
const GIVEBACK_CUSTOM_CONVNEN_ID: &str = "c4f5d6e7-b1a2-3c4d-5e6f-7a8b9c0d1e2f";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde_repr::Serialize_repr, serde_repr::Deserialize_repr, toasty::Embed)]
#[repr(u8)]
pub enum CardPool {
    #[column(variant = 1)]
    FeaturedResonatorConvene = 1,
    #[column(variant = 2)]
    FeaturedWeaponConvene,
    #[column(variant = 3)]
    StandardResonatorConvene,
    #[column(variant = 4)]
    StandardWeaponConvene,
    #[column(variant = 5)]
    NoviceConvene,
    #[column(variant = 6)]
    BeginnerChoiceConvene,
    #[column(variant = 7)]
    GivebackCustomConvene,
}

impl CardPool {
    pub fn pool_id(&self) -> Option<String> {
        match self {
            Self::StandardResonatorConvene => Some(STANDARD_RESONATOR_CONVENE_ID.into()),
            Self::StandardWeaponConvene => Some(STANDARD_WEAPON_CONVENE_ID.into()),
            Self::NoviceConvene => Some(NOVICE_CONVENE_ID.into()),
            Self::BeginnerChoiceConvene => Some(BEGINNER_CHOICE_CONVENE_ID.into()),
            Self::GivebackCustomConvene => Some(GIVEBACK_CUSTOM_CONVNEN_ID.into()),
            _ => None,
        }
    }
}

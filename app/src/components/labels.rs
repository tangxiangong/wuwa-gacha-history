use wuwa_gacha_history::{CardPool, QualityLevel};

pub fn card_pool_label(p: CardPool) -> &'static str {
    match p {
        CardPool::FeaturedResonatorConvene => "限定角色活动",
        CardPool::FeaturedWeaponConvene    => "限定武器活动",
        CardPool::StandardResonatorConvene => "常驻角色",
        CardPool::StandardWeaponConvene    => "常驻武器",
        CardPool::NoviceConvene            => "新手",
        CardPool::BeginnerChoiceConvene    => "新手自选",
        CardPool::GivebackCustomConvene    => "感恩自选",
    }
}

pub fn quality_label(q: QualityLevel) -> &'static str {
    match q {
        QualityLevel::FiveStar  => "5★",
        QualityLevel::FourStar  => "4★",
        QualityLevel::ThreeStar => "3★",
    }
}

/// Tailwind text color class for a quality tier.
pub fn quality_text_class(q: QualityLevel) -> &'static str {
    match q {
        QualityLevel::FiveStar  => "text-star-5",
        QualityLevel::FourStar  => "text-star-4",
        QualityLevel::ThreeStar => "text-star-3",
    }
}

use crate::Int;

pub enum StringPropertyKey{
    Name,
}
pub enum IntPropertyKey{
    X,
    Y,
    Width,
    Height,
    OriginalWidth,
    OriginalHeight,
    HitPoints,
    HitPointsMax,

    Level,
    Strength,
    Constitution,
    Dexterity,
    Intelligence,
    Wisdom,
    Charisma,

    Damage,

    Id,

    DamageResist,
    Armor,
 }
 impl IntPropertyKey{
    /// returns true if the value is a valid value for the Propory
    pub fn check_value(&self, value: Int) -> bool {
        match self {
            IntPropertyKey::X => true,
            IntPropertyKey::Y => true,
            IntPropertyKey::Width => value >= 1,
            IntPropertyKey::Height => value >= 1,
            IntPropertyKey::OriginalWidth => value >= 1,
            IntPropertyKey::OriginalHeight => value >= 1,
            IntPropertyKey::Level => value >= 1,
            IntPropertyKey::Strength => value >= 1,
            IntPropertyKey::Constitution => value >= 1,
            IntPropertyKey::Dexterity => value >= 1,
            IntPropertyKey::Intelligence => value >= 1,
            IntPropertyKey::Wisdom => value >= 1,
            IntPropertyKey::Charisma => value >= 1,
            IntPropertyKey::HitPoints => value >= 0,
            IntPropertyKey::HitPointsMax => value >= 0,

            IntPropertyKey::Damage => value >= 0,
            IntPropertyKey::DamageResist => value >= 0 && value <= 800,
            IntPropertyKey::Armor => value >= 0,
            IntPropertyKey::Id => true,
        }
    }
 }
pub enum PricesPropertyKey{
    Prices,
}
pub enum DemandsPropertyKey{
    Demands,
}
pub enum BuildingListPropertyKey{
    Buildings,
}
pub enum MiscPropertyKey{
    CONDITIONS,
    FOOD,
    WATER,
    ENERGY,
    SOCIAL,
    AlcoholLevel,
    GOLD,
    CreatureType,
    GROUP,
}

pub enum SkillPropertyKey{
    
}
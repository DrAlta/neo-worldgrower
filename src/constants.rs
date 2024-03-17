use crate::Int;
use crate::rustations::{PropertyKey, property_keys::*};
use crate::attribute::building_list::BuildingList;
use crate::attribute::demands::Demands;
use crate::attribute::prices::Prices;


pub const NAME: PropertyKey<StringPropertyKey, String> = StringPropertyKey::Name;
pub const LEVEL: PropertyKey<IntPropertyKey, Int> = IntPropertyKey::Level;
pub const STRENGTH: PropertyKey<IntPropertyKey, Int> = IntPropertyKey::Strength;
pub const CONSTITUTION: PropertyKey<IntPropertyKey, Int> = IntPropertyKey::Constitution;
pub const DEXTERITY: PropertyKey<IntPropertyKey, Int> = IntPropertyKey::Dexterity;
pub const INTELLIGENCE: PropertyKey<IntPropertyKey, Int> = IntPropertyKey::Intelligence;
pub const WISDOM: PropertyKey<IntPropertyKey, Int> = IntPropertyKey::Wisdom;
pub const CHARISMA: PropertyKey<IntPropertyKey, Int> = IntPropertyKey::Charisma;
pub const PRICES: PropertyKey<PricesPropertyKey, Prices> = PricesPropertyKey::Prices;
pub const DEMANDS: PropertyKey<DemandsPropertyKey, Demands> = DemandsPropertyKey::Demands;
pub const BUILDINGS: PropertyKey<BuildingListPropertyKey, BuildingList> = BuildingListPropertyKey::Buildings;
pub const ARMOR: PropertyKey = PropertyKey::ARMOR;
pub const HIT_POINTS: PropertyKey = PropertyKey::HitPoints;
pub const DAMAGE_RESIST: PropertyKey = PropertyKey::DamageResist;
pub const DAMAGE: PropertyKey = PropertyKey::DAMAGE;

pub const ID: PropertyKey<IntPropertyKey, Int> = IntPropertyKey::Id;

pub const CONDITIONS: PropertyKey = PropertyKey::CONDITIONS;
pub const X: PropertyKey = PropertyKey::X;
pub const Y: PropertyKey = PropertyKey::Y;
pub const WIDTH: PropertyKey = PropertyKey::WIDTH;
pub const HEIGHT: PropertyKey = PropertyKey::HEIGHT;
pub const FOOD: PropertyKey = PropertyKey::FOOD;
pub const WATER: PropertyKey = PropertyKey::WATER;
pub const ENERGY: PropertyKey = PropertyKey::ENERGY;
pub const SOCIAL: PropertyKey = PropertyKey::SOCIAL;
pub const ALCOHOL_LEVEL: PropertyKey = PropertyKey::AlcoholLevel;
pub const GOLD: PropertyKey = PropertyKey::GOLD;
pub const CREATURE_TYPE: PropertyKey = PropertyKey::CreatureType;
pub const GROUP: PropertyKey = PropertyKey::GROUP;

use crate::{WorldObjectID, Int};
use crate::attribute::building_list::BuildingList;
use crate::attribute::demands::Demands;
use crate::attribute::id_list::IdList;
use crate::attribute::prices::Prices;
use crate::attribute::world_object_properties::WorldObjectProperties;
use crate::condition::conditions::Conditions;
use crate::constants;
use crate::rustations::{GetterSetter, PropertyKey};
use crate::world_object::WorldObject;

pub fn create_skilled_world_object<WO: WorldObject>(id: WorldObjectID) -> WO {
    let mut properties = WorldObjectProperties::new();
    properties.insert(constants::NAME, "worldObject".to_string());
    properties.insert(constants::LEVEL, 1);
    properties.insert(constants::STRENGTH, 10);
    properties.insert(constants::CONSTITUTION, 10);
    properties.insert(constants::DEXTERITY, 10);
    properties.insert(constants::INTELLIGENCE, 10);
    properties.insert(constants::WISDOM, 10);
    properties.insert(constants::CHARISMA, 10);
    properties.insert(constants::PRICES, Prices::new());
    properties.insert(constants::DEMANDS, Demands::new());
    properties.insert(constants::BUILDINGS, BuildingList::new());
    properties.insert(constants::ARMOR, 10);
    properties.insert(constants::HIT_POINTS, 10);
    properties.insert(constants::DAMAGE_RESIST, 0);
    properties.insert(constants::DAMAGE, 4);
    properties.insert(constants::CONDITIONS, Conditions::new());
    properties.insert(constants::X, 0);
    properties.insert(constants::Y, 0);
    properties.insert(constants::WIDTH, 1);
    properties.insert(constants::HEIGHT, 1);
    properties.insert(constants::FOOD, 1000);
    properties.insert(constants::WATER, 1000);
    properties.insert(constants::ENERGY, 1000);
    properties.insert(constants::SOCIAL, 1000);
    properties.insert(constants::ALCOHOL_LEVEL, 1000);
    properties.insert(constants::GOLD, 1000);
    properties.insert(constants::CREATURE_TYPE, crate::creaturetype::creature_type::HUMAN_CREATURE_TYPE);
    properties.insert(constants::GROUP, IdList::new());
    SkillUtils.addAllSkills(properties);
    let world_object = create_world_object_id_properties(id, properties);
    return world_object;
}

pub fn  simple_create_skilled_world_object<WO: WorldObject, PK, T>(id: WorldObjectID, property_key: PropertyKey<PK, T>, value: T) -> WO {
    let world_object = create_skilled_world_object(id);
    world_object.set_property(property_key, value);
    return world_object;
}

pub fn create_world_object_id_properties<WO: WorldObject>(id: Int, properties: WorldObjectProperties) -> WO {
		properties.insert(constants::ID, id);
		WorldObjectImpl(properties, Actions.ALL_ACTIONS, None);
}
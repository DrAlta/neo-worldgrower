use crate::WorldObjectID;
use crate::constants;
use crate::world_object::WorldObject;

pub fn create_skilled_world_object(id: WorldObjectID) -> WorldObject{
    Map<ManagedProperty<?>, Object> properties = new HashMap<>();
    properties.put(Constants.NAME, "worldObject");
    properties.put(Constants.LEVEL, 1);
    properties.put(Constants.STRENGTH, 10);
    properties.put(Constants.CONSTITUTION, 10);
    properties.put(Constants.DEXTERITY, 10);
    properties.put(Constants.INTELLIGENCE, 10);
    properties.put(Constants.WISDOM, 10);
    properties.put(Constants.CHARISMA, 10);
    properties.put(Constants.PRICES, new Prices());
    properties.put(Constants.DEMANDS, new Demands());
    properties.put(Constants.BUILDINGS, new BuildingList());
    properties.put(Constants.ARMOR, 10);
    properties.put(Constants.HIT_POINTS, 10);
    properties.put(Constants.DAMAGE_RESIST, 0);
    properties.put(Constants.DAMAGE, 4);
    properties.put(Constants.CONDITIONS, new Conditions());
    properties.put(Constants.X, 0);
    properties.put(Constants.Y, 0);
    properties.put(Constants.WIDTH, 1);
    properties.put(Constants.HEIGHT, 1);
    properties.put(Constants.FOOD, 1000);
    properties.put(Constants.WATER, 1000);
    properties.put(Constants.ENERGY, 1000);
    properties.put(Constants.SOCIAL, 1000);
    properties.put(Constants.ALCOHOL_LEVEL, 1000);
    properties.put(Constants.GOLD, 1000);
    properties.put(Constants.CREATURE_TYPE, CreatureType.HUMAN_CREATURE_TYPE);
    properties.put(Constants.GROUP, new IdList());
    SkillUtils.addAllSkills(properties);
    WorldObject worldObject = createWorldObject(id, properties);
    return worldObject;
}

pub fn  simple_create_skilled_world_object<T>(id: WorldObjectID, property_key: PropertyKey, value: T) -> WorldObject {
    let world_object = create_skilled_world_object(id);
    world_object.set_property(propert_key, value);
    return world_object;
}
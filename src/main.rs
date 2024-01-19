type Int = i32;
type WorldObjectID = usize;
// start finished mods
mod rustations;
// end finished mods
// start placeholder mods
mod attribute;
mod constants;
mod world;
mod world_impl;
mod world_object;
mod terrain;
mod terrain_impl;
mod test_utils;
// end placeholder mods


fn main() {
    //foo::food_goal_test();
    println!("Hello, world!");
}

mod foo {

    use crate::world::World;
    use crate::world_impl::WorldImpl;
    use crate::world_object::WorldObject;
    use crate::terrain::Terrain;
    use crate::terrain_impl::TerrainImpl;
    use crate::test_utils;
/*
    fn create_performer() -> WorldObject{
        let performer = test_utils.simple_create_skilled_world_object(1, constants.PERSONALITY, Personality::new())
        performer.set_property(constants.X, 0);
        performer.set_property(constants.Y, 0);
        performer.set_property(constants.WIDTH, 1);
        performer.set_property(constants.HEIGHT, 1);
        return performer;
    }
    /////////////////////
    //////////////////////////////////
    fn food_goal_test(){
        let mut world = WorldImpl::simple_new(1, 1, None, None);
        let performer = create_performer();
        
        performer.removeProperty(Constants.DEMANDS);
        goal.defaultGoalMetOrNot(performer, world, false, Constants.FOOD);
        assertEquals(null, performer.getProperty(Constants.DEMANDS));
        
        performer.setProperty(Constants.DEMANDS, new Demands());
        goal.defaultGoalMetOrNot(performer, world, false, Constants.FOOD);
        assertEquals(1, performer.getProperty(Constants.DEMANDS).count(Constants.FOOD));
        
        goal.defaultGoalMetOrNot(performer, world, true, Constants.FOOD);
        assertEquals(0, performer.getProperty(Constants.DEMANDS).count(Constants.FOOD));
        
        try {
            goal.defaultGoalMetOrNot(performer, world, false, Constants.CONSTITUTION);
            fail("method should fail");
        } catch(IllegalStateException e) {
            assertEquals("property CON isn't found in list of possible demands", e.getMessage());
        }
    }
    */
}
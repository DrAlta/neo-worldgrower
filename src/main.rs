type Int = i32;
// start finished mods
// end finished mods
// start placeholder mods
mod world;
mod terrain;
mod terrain_impl;
// end placeholder mods


fn main() {
    food_goal_test();
    println!("Hello, world!");
}


use world::World;
use world_impl::WorldImpl;
use terrain::Terrain;
use terrain_impl::TerrainImpl;
fn food_goal_test(){
    let mut world = World::WorldImpl(1, 1, None, None);
    WorldObject performer = createPerformer();
    
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
use crate::world::World;
use crate::terrain::Terrain;
use crate::terrain_impl::TerrainImpl;

pub struct WorldImpl<T:Terrain, WT:WorldOnTurn> {
    terrain: T,  
    dungeon_master: Option<DungeonMaster>,  
    world_on_turn: Option<WT>,
}

impl<WT:WorldOnTurn> WorldImpl<TerrainImpl, WT: WorldOnTurn> {
    pub fn simple_new(width: Int, height: Int,  dungeon_master: Option<DungeonMaster>,  world_on_turn: Option<WT>) -> Self {
        Self::new(
            TerrainImpl::new(width, height, TerrainMapper::new()), 
            dungeon_master, 
            world_on_turn
        )
    }
}


impl<T:Terrain, WT:WorldOnTurn> WorldImpl<T, WT> {
    pub fn new(terrain:T,  dungeon_master: Option<DungeonMaster>,  world_on_turn: Option<WT>) -> Self {
    }
}

impl<T:Terrain, WT:WorldOnTurn> World for WorldImpl<T, WT> {

}
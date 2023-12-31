use bevy::prelude::*;
use bevy::utils::HashMap;


#[derive(Component)]
pub struct SavingName(pub String);

#[derive(Component, Default, Resource)]
pub struct ChunkList {
    pub list: HashMap<IVec2, Entity>
}

impl ChunkList {
    pub fn new() -> Self {
        Self {
            list: HashMap::new()
        }
    }
}

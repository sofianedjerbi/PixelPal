use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};


#[derive(Component)]
pub struct SavingName(pub String);

#[derive(Component, Default)]
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


#[derive(Component, Default, Resource, Deref, DerefMut)]
pub struct IndexChunkList {
    pub list: HashSet<IVec2>
}

impl IndexChunkList {
    pub fn new() -> Self {
        Self {
            list: HashSet::new()
        }
    }
}

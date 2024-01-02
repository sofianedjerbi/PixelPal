use std::fmt;

use bevy::prelude::*;
use bevy::utils::HashMap;


#[derive(Component, Deref)]
pub struct ReliefLevel(pub u32);

impl fmt::Display for ReliefLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Component)]
pub struct SavingName(pub String);

#[derive(Component, Resource, Default, Deref, DerefMut)]
pub struct ChunkMap(pub HashMap<IVec2, Entity>);

impl ChunkMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

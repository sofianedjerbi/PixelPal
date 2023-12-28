use bevy::{prelude::*, utils::HashSet};


#[derive(Component)]
pub struct SavingName(pub String);

#[derive(Component, Default, Debug, Resource)]
pub struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}

use std::fmt;

use bevy::ecs::system::CommandQueue;
use bevy::prelude::*;
use bevy::tasks::Task;
use bevy::utils::HashMap;
use bevy_ecs_tilemap::map::TilemapTexture;

/// Resource representing the main tilemap texture.
#[derive(Resource, Deref, DerefMut)]
pub struct MainTilemapTexture(
    /// The handle to the tilemap texture.
    Option<TilemapTexture>,
);

impl MainTilemapTexture {
    /// Creates a new `MainTilemapTexture` with no texture handle.
    pub fn default() -> Self {
        Self(None)
    }

    /// Sets the tilemap texture handle.
    ///
    /// # Arguments
    ///
    /// * `image` - The handle to the image resource.
    pub fn set_handle(&mut self, image: Handle<Image>) {
        self.0 = Some(TilemapTexture::Single(image));
    }

    /// Retrieves the tilemap texture.
    pub fn get(&self) -> TilemapTexture {
        self.0.clone().unwrap()
    }
}

/// Component representing the relief level of a tile.
#[derive(Component, Deref)]
pub struct ReliefLevel(
    /// The relief level value.
    pub u32,
);

impl fmt::Display for ReliefLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Component representing the name used for saving.
#[derive(Component)]
pub struct SavingName(
    /// The name used for saving.
    pub String,
);

/// Component representing the chunk map.
#[derive(Component, Resource, Default, Deref, DerefMut)]
pub struct ChunkMap(
    /// A HashMap that maps chunk positions to their respective entities.
    pub HashMap<IVec2, (Entity, Entity)>,
);

impl ChunkMap {
    /// Creates a new `ChunkMap`.
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

/// Component representing the ID of a layer.
#[derive(Component, Clone, Deref)]
pub struct LayerId(
    /// The ID of the layer.
    pub u32,
);

/// Component representing a chunk task with a command queue.
#[derive(Component, Deref)]
pub struct ChunkTask(pub Task<CommandQueue>);

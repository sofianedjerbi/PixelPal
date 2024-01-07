use std::fmt;

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_ecs_tilemap::map::TilemapTexture;


#[derive(Resource, Deref, DerefMut)]
pub struct MainTilemapTexture(Option<TilemapTexture>);

impl MainTilemapTexture {
    pub fn default() -> Self {
        Self(None)
    }

    pub fn set_handle(&mut self, image: Handle<Image>) {
        self.0 = Some(TilemapTexture::Single(image));
    }

    pub fn get(&self) -> TilemapTexture {
        self.0.clone().unwrap()
    }
}

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
pub struct ChunkMap(pub HashMap<IVec2, (Entity, Entity)>);

impl ChunkMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

#[derive(Component, Clone, Deref)]
pub struct LayerId(pub u32);

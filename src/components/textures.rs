use bevy::prelude::*;
use phf::Map;


#[derive(Component, Deref)]
pub struct TextureIDProbabilityPHF(
    pub Map<u32, u32>
);

#[derive(Component, Deref)]
pub struct TextureIDOffsetPHF(
    pub Map<u32, u32>
);

#[derive(Component, Deref)]
pub struct TextureReliefIDsPHF(
    pub Map<u32, &'static TextureIDProbabilityPHF>
);

#[derive(Component, Deref)]
pub struct TextureCornerIDsPHF(
    pub Map<u32, u32>
);

#[derive(Component, DerefMut, Deref)]
pub struct TilesetOffset(pub Vec2);


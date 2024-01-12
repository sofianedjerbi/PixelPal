use bevy::prelude::*;
use phf::Map;

/// Component representing a Probability Map of Texture IDs.
#[derive(Component, Deref)]
pub struct TextureIDProbabilityPHF(
    /// A Map of Texture IDs to their respective probabilities.
    pub Map<u32, u32>
);

/// Component representing an Offset Map of Texture IDs.
#[derive(Component, Deref)]
pub struct TextureIDOffsetPHF(
    /// A Map of Texture IDs to their respective offsets.
    pub Map<u32, u32>
);

/// Component representing a Map of Relief Texture IDs.
#[derive(Component, Deref)]
pub struct TextureReliefIDsPHF(
    /// A Map of Texture IDs to their corresponding probability maps.
    pub Map<u32, &'static TextureIDProbabilityPHF>
);

/// Component representing a Map of Texture Corner IDs.
#[derive(Component, Deref)]
pub struct TextureCornerIDsPHF(
    /// A Map of Texture Corner IDs to their respective texture IDs.
    pub Map<u32, u32>
);

/// Component representing the offset of a Tileset.
#[derive(Component, DerefMut, Deref)]
pub struct TilesetOffset(
    /// The offset in 2D space.
    pub Vec2
);

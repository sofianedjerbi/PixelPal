use benimator::*;
use bevy::prelude::*;
use bevy::utils::HashMap;

use super::action::Action;


#[derive(Default, Component, Deref, DerefMut)]
pub struct AnimationState(pub benimator::State);

#[derive(Component, Clone, Deref)]
pub struct ActionAnimationMap(
    pub HashMap<Action, Animation>
);

#[derive(Component, Clone, Deref)]
pub struct TileAnimationMap(
    pub HashMap<(u32, u32), Animation> // (layer, tile)
);

impl ActionAnimationMap {
    pub fn lookup(&self, action: &Action) -> &Animation {
        self.0.get(action).unwrap()
    }
}

pub struct AnimationSpriteGrid {
    pub size: Vec2,
    pub columns: usize,
    pub rows: usize,
    pub padding: Option<Vec2>,
    pub offset: Option<Vec2>,
}

impl AnimationSpriteGrid {
    pub fn default() -> Self {
        Self {
            size: Vec2::new(0., 0.),
            columns: 0,
            rows: 0,
            padding: None,
            offset: None
        }
    }

    pub fn to_atlas(
        &self,
        texture: Handle<Image>
    ) -> TextureAtlas {
        TextureAtlas::from_grid(
            texture,
            self.size,
            self.columns,
            self.rows,
            self.padding,
            self.offset
        )
    }
}

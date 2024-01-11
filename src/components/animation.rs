use benimator::*;
use bevy::prelude::*;
use bevy::utils::HashMap;

use super::action::Action;


#[derive(Default, Component, Deref, DerefMut)]
pub struct AnimationState(pub benimator::State);


#[derive(Component, Clone)]
pub struct FixedAnimation {
    pub start: u32,
    pub end: u32,
    pub speed: f32
}

impl FixedAnimation {
    pub fn new(
        start: u32,
        end: u32,
        speed: f32
    ) -> Self {
        Self { start, end, speed }
    }
}

#[derive(Component, Clone, Deref)]
pub struct ActionAnimationMap(
    pub HashMap<Action, Animation>
);

#[derive(Component, Clone, Deref)]
pub struct TileAnimationMap(
    pub HashMap<(u32, u32), FixedAnimation> // (layer, tile)
);

impl TileAnimationMap {
    pub fn lookup(&self, position: &(u32, u32)) -> Option<FixedAnimation> {
        self.get(position).cloned()
    }
}

impl ActionAnimationMap {
    pub fn lookup(&self, action: &Action) -> &Animation {
        self.get(action).unwrap()
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

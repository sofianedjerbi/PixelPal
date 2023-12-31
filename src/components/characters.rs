use bevy::prelude::*;


#[derive(Component, DerefMut, Deref)]
pub struct Health(pub u8);

#[derive(Component, DerefMut, Deref)]
pub struct Busy(pub bool);

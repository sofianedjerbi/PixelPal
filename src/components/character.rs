use std::sync::atomic::AtomicBool;

use bevy::prelude::*;

#[derive(Component, DerefMut, Deref)]
pub struct Health(pub u8);

#[derive(Component, DerefMut, Deref)]
pub struct Busy(pub AtomicBool);

/// Component indicating that an entity is a user.
#[derive(Component)]
pub struct IsUser;

/// Component indicating that an entity is a bot.
#[derive(Component)]
pub struct IsBot;

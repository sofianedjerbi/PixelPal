use bevy::prelude::*;
use crate::components::characters::*;

use super::map::TILE;


// PLAYERS
pub const PLAYER_HEALTH: Health = Health(100);
// User
pub const USER_SPAWN: Vec2 = Vec2::new(TILE * 0., TILE * 0.);
// Mittens
pub const MITTENS_SPAWN: Vec2 = Vec2::new(TILE * 4., TILE * 0.);

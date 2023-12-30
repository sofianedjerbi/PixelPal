use bevy::log;
use bevy::math::Vec3;
use bevy::prelude::*;

use crate::components::action::*;
use crate::components::characters::*;
use crate::constants::characters::ACTION_TRANSFORMATION;
use crate::constants::tps::MOVEMENT_TPS;


pub fn move_characters(
    mut query: Query<(
        &mut Transform,
        &mut Busy, 
        &mut Action
    ), With<IsUser>>,
) {
    for (
        mut transform,
        mut busy,
        mut action
    ) in query.iter_mut() {
        log::error!("{:?}", transform.translation);
        if !**busy { continue; }

        action.timer.tick(MOVEMENT_TPS);

        if action.timer.finished() {
            **busy = false;
            // Fix f32 residus
            transform.translation = transform.translation.round();
            return
        }

        let action_transform = ACTION_TRANSFORMATION(&action);
        let movement = action_transform 
            * Vec3::splat(MOVEMENT_TPS.as_secs_f32() 
            / action.timer.duration().as_secs_f32());

        transform.translation += movement;
    }
}

pub fn camera_follow_player(
    player_query: Query<&Transform, With<IsUser>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<IsUser>)>,
) {
    
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation = player_transform.translation;
}

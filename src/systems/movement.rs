use bevy::prelude::*;

use crate::components::action::*;
use crate::components::characters::*;
use crate::components::flags::IsBot;
use crate::components::flags::IsUser;
use crate::components::gpt::GPTAgent;
use crate::constants::action::ACTION_TICK_FREQUENCY;


pub fn move_characters(
    mut query: Query<(
        &mut Transform,
        &mut Busy, 
        &Action,
        &mut ActionTimer
    )>,
) {
    for (
        mut transform,
        mut busy,
        action,
        mut timer
    ) in query.iter_mut() {
        if !**busy { continue; }

        timer.tick(ACTION_TICK_FREQUENCY);

        if timer.finished() {
            **busy = false;
            transform.translation = transform.translation.round();
            return
        }

        let movement = action.get_transformation() 
            * Vec3::splat(ACTION_TICK_FREQUENCY.as_secs_f32() 
            / timer.duration().as_secs_f32());

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

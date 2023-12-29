use bevy::input::Input;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::render::camera::Camera;

use crate::components::characters::Busy;
use crate::components::animation::*;

//TODO: Filter player only in this function, With<PlayerBundle>
pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Busy, &mut AnimationAction)>
) {
    for (
        mut busy,
        mut current_animation
    ) in query.iter_mut() {
        //if **busy { return }

        if keyboard_input.pressed(KeyCode::Q) {
            current_animation.action_type = ActionType::Walking;
            current_animation.direction = AnimationDirection::Left;
            **busy = true;
        }
        else if keyboard_input.pressed(KeyCode::D) {
            current_animation.action_type = ActionType::Walking;
            current_animation.direction = AnimationDirection::Right;
            **busy = true;
        }
        else if keyboard_input.pressed(KeyCode::Z) {
            current_animation.action_type = ActionType::Walking;
            current_animation.direction = AnimationDirection::Up;
            **busy = true;
        }
        else if keyboard_input.pressed(KeyCode::S) {
            current_animation.action_type = ActionType::Walking;
            current_animation.direction = AnimationDirection::Down;
            **busy = true;
        }
        else {
            current_animation.action_type = ActionType::Standing;
        }
    }
}

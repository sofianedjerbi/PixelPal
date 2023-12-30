use bevy::input::Input;
use bevy::prelude::*;

use crate::components::characters::*;
use crate::components::action::*;
use crate::constants::characters::PLAYER_ACTION_TIMER;


pub fn handle_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Busy, &mut Action), With<IsUser>>
) {
    for (
        mut busy,
        mut action,
    ) in query.iter_mut() {
        if **busy { return }

        let new_state = 
        match keyboard_input.get_pressed().next() {
            Some(KeyCode::Q) =>
                Some((ActionType::Walking, ActionDirection::Left)),
            Some(KeyCode::D) =>
                Some((ActionType::Walking, ActionDirection::Right)),
            Some(KeyCode::Z) =>
                Some((ActionType::Walking, ActionDirection::Up)),
            Some(KeyCode::S) =>
                Some((ActionType::Walking, ActionDirection::Down)),
            _ => None,
        };

        if let Some((
            action_type,
            direction
        )) = new_state {
            action.action_type = action_type;
            action.direction = direction;
            action.timer = PLAYER_ACTION_TIMER(&action.action_type);
            **busy = true;
        } else {
            action.action_type = ActionType::Standing;
        }
    }
}

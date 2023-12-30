use bevy::input::Input;
use bevy::prelude::*;

use crate::components::characters::*;
use crate::components::action::*;
use crate::constants::action::PLAYER_ACTION_DEFAULT;


pub fn handle_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut Busy,
        &mut Action,
        &mut ActionTimer,
        &ActionDurationPHF
    ), With<IsUser>>
) {
    for (
        mut busy,
        mut action,
        mut timer,
        duration
    ) in query.iter_mut() {
        if **busy { return }

        let new_state = 
        match keyboard_input.get_pressed().next() {
            Some(KeyCode::Q) =>
                Some((ActionKind::Walking, ActionDirection::Left)),
            Some(KeyCode::D) =>
                Some((ActionKind::Walking, ActionDirection::Right)),
            Some(KeyCode::Z) =>
                Some((ActionKind::Walking, ActionDirection::Up)),
            Some(KeyCode::S) =>
                Some((ActionKind::Walking, ActionDirection::Down)),
            _ => None,
        };

        if let Some((
            kind,
            direction
        )) = new_state {
            action.kind = kind;
            action.direction = direction;
            *timer = duration.generate_timer(&action);
            **busy = true;
        } else {
            action.kind = PLAYER_ACTION_DEFAULT.kind;
        }
    }
}

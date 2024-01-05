use bevy::input::Input;
use bevy::prelude::*;

use crate::components::characters::*;
use crate::components::action::*;
use crate::components::gpt::GPTAgent;
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

        let action_kind = if keyboard_input.pressed(KeyCode::ShiftLeft) 
                                      || keyboard_input.pressed(KeyCode::ShiftRight) {
            ActionKind::Run
        } else {
            ActionKind::Walk
        };
        
        let new_state = if keyboard_input.pressed(KeyCode::S) {
            Some((action_kind, ActionDirection::Down))
        } else if keyboard_input.pressed(KeyCode::Z) {
            Some((action_kind, ActionDirection::Up))
        } else if keyboard_input.pressed(KeyCode::Q) {
            Some((action_kind, ActionDirection::Left))
        } else if keyboard_input.pressed(KeyCode::D) {
            Some((action_kind, ActionDirection::Right))
        } else {
            None
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


pub fn handle_bot_input(
    mut query: Query<(
        &mut Busy,
        &mut Action,
        &mut ActionTimer,
        &ActionDurationPHF,
        &GPTAgent
    ), With<IsBot>>
) {
    for (
        mut busy,
        mut action,
        mut timer,
        duration,
        agent
    ) in query.iter_mut() {
        if **busy { return }

        if let Ok(mut queue) = agent.action_queue.try_lock() {
            if let Some(new_action) = queue.pop_front() {
                *action = new_action.clone();
                *timer = duration.generate_timer(&action);
                **busy = true;
                return;
            }
        }
        action.kind = PLAYER_ACTION_DEFAULT.kind;
    }
}

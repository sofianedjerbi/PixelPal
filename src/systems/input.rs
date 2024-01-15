use bevy::input::Input;
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TileStorage;

use crate::components::action::*;
use crate::components::characters::*;
use crate::components::gpt::GPTAgent;
use crate::components::map::ChunkMap;
use crate::components::map::ReliefLevel;
use crate::components::textures::TilesetOffset;
use crate::constants::action::PLAYER_ACTION_DEFAULT;
use crate::util::position::player_tile_pos;
use crate::util::position::relative_tile_pos;
use crate::util::position::tile_pos_to_chunk_pos;

// Define a type for player character queries
type PlayerCharacterQuery<'a> = (
    &'a mut Busy,
    &'a mut Action,
    &'a mut ActionTimer,
    &'a ActionDurationPHF,
    &'a Transform,
    &'a TilesetOffset,
);

// Define a type for bot character queries
type BotCharacterQuery<'a> = (
    &'a mut Busy,
    &'a mut Action,
    &'a mut ActionTimer,
    &'a ActionDurationPHF,
    &'a Transform,
    &'a TilesetOffset,
    &'a GPTAgent,
);

/// Key codes
const KEY_UP: [KeyCode; 2] = [KeyCode::Up, KeyCode::W];
const KEY_LEFT: [KeyCode; 2] = [KeyCode::Left, KeyCode::A];
const KEY_DOWN: [KeyCode; 2] = [KeyCode::Down, KeyCode::S];
const KEY_RIGHT: [KeyCode; 2] = [KeyCode::Right, KeyCode::D];
const KEY_RUN: [KeyCode; 2] = [KeyCode::ShiftLeft, KeyCode::ShiftRight];

/// Handles keyboard input for player characters.
///
/// # Parameters
/// - `keyboard_input`: The current state of the keyboard.
/// - `query`: Query for accessing and modifying the components related to user actions.
/// - `chunk_map`: Resource providing the game's chunk map.
/// - `chunk_query`: Query for accessing tile storage data.
/// - `tile_query`: Query for accessing relief level of tiles.
///
/// This function processes the keyboard inputs and updates the actions of the player character accordingly.
pub fn handle_input(
    mut query: Query<PlayerCharacterQuery, With<IsUser>>,
    keyboard_input: Res<Input<KeyCode>>,
    chunk_map: Res<ChunkMap>,
    chunk_query: Query<&TileStorage>,
    tile_query: Query<&ReliefLevel>,
) {
    for (mut busy, mut action, mut timer, duration, transform, offset) in query.iter_mut() {
        if **busy {
            return;
        }

        let action_kind = if keyboard_input.any_pressed(KEY_RUN) {
            ActionKind::Run
        } else {
            ActionKind::Walk
        };

        let new_action_option = if keyboard_input.any_pressed(KEY_DOWN) {
            Some(Action::new(action_kind, ActionDirection::Down))
        } else if keyboard_input.any_pressed(KEY_UP) {
            Some(Action::new(action_kind, ActionDirection::Up))
        } else if keyboard_input.any_pressed(KEY_LEFT) {
            Some(Action::new(action_kind, ActionDirection::Left))
        } else if keyboard_input.any_pressed(KEY_RIGHT) {
            Some(Action::new(action_kind, ActionDirection::Right))
        } else {
            None
        };

        match new_action_option {
            Some(new_action)
                if is_action_possible(
                    &new_action,
                    transform,
                    offset,
                    &chunk_map,
                    &chunk_query,
                    &tile_query,
                ) =>
            {
                *action = new_action;
                *timer = duration.generate_timer(&action);
                **busy = true;
            }
            _ => action.kind = PLAYER_ACTION_DEFAULT.kind,
        }
    }
}

/// Handles input for bot characters.
///
/// # Parameters
/// - `query`: Query for accessing and modifying the components related to bot actions.
/// - `chunk_map`: Resource providing the game's chunk map.
/// - `chunk_query`: Query for accessing tile storage data.
/// - `tile_query`: Query for accessing relief level of tiles.
///
/// This function processes the actions queued for bot characters and updates their actions accordingly.
pub fn handle_bot_input(
    mut query: Query<BotCharacterQuery, With<IsBot>>,
    chunk_map: Res<ChunkMap>,
    chunk_query: Query<&TileStorage>,
    tile_query: Query<&ReliefLevel>,
) {
    for (mut busy, mut action, mut timer, duration, transform, offset, agent) in query.iter_mut() {
        if **busy {
            return;
        }

        if let Ok(mut queue) = agent.action_queue.try_write() {
            if let Some(new_action) = queue.pop_front() {
                if is_action_possible(
                    &new_action,
                    transform,
                    offset,
                    &chunk_map,
                    &chunk_query,
                    &tile_query,
                ) {
                    *action = new_action;
                    *timer = duration.generate_timer(&action);
                    **busy = true;
                    return;
                }
            }
        }
        action.kind = PLAYER_ACTION_DEFAULT.kind;
    }
}

/// Determines if an action is possible based on the current game state.
///
/// # Parameters
/// - `action`: The action to be evaluated.
/// - `transform`: The current transform of the entity.
/// - `offset`: Tileset offset for calculating positions.
/// - `chunk_map`: Resource providing the game's chunk map.
/// - `chunk_query`: Query for accessing tile storage data.
/// - `tile_query`: Query for accessing relief level of tiles.
///
/// # Returns
/// Returns `true` if the action is possible, `false` otherwise.
///
/// This function checks if the specified action can be performed by the entity based on its current state and the state of the game world.
fn is_action_possible(
    action: &Action,
    transform: &Transform,
    offset: &TilesetOffset,
    chunk_map: &Res<ChunkMap>,
    chunk_query: &Query<&TileStorage>,
    tile_query: &Query<&ReliefLevel>,
) -> bool {
    if !matches!(action.kind, ActionKind::Walk | ActionKind::Run) {
        return true;
    }
    let position = &player_tile_pos(transform, offset);
    let position_relative = relative_tile_pos(position);

    let target_pos = *position + action.get_raw_transformation();
    let target_chunk_pos = tile_pos_to_chunk_pos(&target_pos);

    let current_chunk_pos = tile_pos_to_chunk_pos(position);
    let (layer, _) = chunk_map
        .get(&current_chunk_pos)
        .expect("Unable to get level 0 layer!");
    let tile_storage = chunk_query
        .get(*layer)
        .expect("Unable to get level 0 tile storage!");
    let tile_entiy = tile_storage
        .get(&position_relative)
        .expect("Unable to get level 0 tile entity!");
    let level0 = **tile_query
        .get(tile_entiy)
        .expect("Unable to get level 0 value!") as f32;

    if let Some((layer, _)) = chunk_map.get(&target_chunk_pos) {
        let tile_storage = chunk_query
            .get(*layer)
            .expect("Unable to get level 1 tile storage!");
        let target_relative = relative_tile_pos(&target_pos);
        if let Some(tile_entiy) = tile_storage.get(&target_relative) {
            let level1 = **tile_query
                .get(tile_entiy)
                .expect("Unable to get level 1 value!") as f32;
            return level0 == level1;
        } else {
            return true;
        }
    }
    false
}

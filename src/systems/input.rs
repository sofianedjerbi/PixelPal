use bevy::input::Input;
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TileStorage;

use crate::components::characters::*;
use crate::components::action::*;
use crate::components::gpt::GPTAgent;
use crate::components::map::ChunkMap;
use crate::components::textures::TilesetOffset;
use crate::constants::action::PLAYER_ACTION_DEFAULT;
use crate::components::map::ReliefLevel;
use crate::util::position::player_tile_pos;
use crate::util::position::relative_tile_pos;
use crate::util::position::tile_pos_to_chunk_pos;


pub fn handle_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut Busy,
        &mut Action,
        &mut ActionTimer,
        &ActionDurationPHF,
        &Transform,
        &TilesetOffset
    ), With<IsUser>>,
    chunk_map: Res<ChunkMap>,
    chunk_query: Query<&TileStorage>,
    tile_query: Query<&ReliefLevel>,
) {
    for (
        mut busy,
        mut action,
        mut timer,
        duration,
        transform,
        offset
    ) in query.iter_mut() {
        if **busy { return }

        let action_kind = if keyboard_input.pressed(KeyCode::ShiftLeft) 
                                      || keyboard_input.pressed(KeyCode::ShiftRight) {
            ActionKind::Run
        } else {
            ActionKind::Walk
        };
        
        let new_action_option = if keyboard_input.pressed(KeyCode::S) {
            Some(Action::new(action_kind, ActionDirection::Down))
        } else if keyboard_input.pressed(KeyCode::Z) {
            Some(Action::new(action_kind, ActionDirection::Up))
        } else if keyboard_input.pressed(KeyCode::Q) {
            Some(Action::new(action_kind, ActionDirection::Left))
        } else if keyboard_input.pressed(KeyCode::D) {
            Some(Action::new(action_kind, ActionDirection::Right))
        } else {
            None
        };

        match new_action_option {
            Some(new_action) if is_action_possible(
                &new_action,
                transform,
                offset,
                &chunk_map,
                &chunk_query,
                &tile_query
            ) => {
                *action = new_action;
                *timer = duration.generate_timer(&action);
                **busy = true;
            }
            _ => action.kind = PLAYER_ACTION_DEFAULT.kind,
        }
        
    }
}


pub fn handle_bot_input(
    mut query: Query<(
        &mut Busy,
        &mut Action,
        &mut ActionTimer,
        &ActionDurationPHF,
        &Transform,
        &TilesetOffset,
        &GPTAgent
    ), With<IsBot>>,
    chunk_map: Res<ChunkMap>,
    chunk_query: Query<&TileStorage>,
    tile_query: Query<&ReliefLevel>,
) {
    for (
        mut busy,
        mut action,
        mut timer,
        duration,
        transform,
        offset,
        agent
    ) in query.iter_mut() {
        if **busy { return }

        if let Ok(mut queue) = agent.action_queue.try_lock() {
            if let Some(new_action) = queue.pop_front() {
                if is_action_possible(
                    &new_action,
                    transform,
                    offset,
                    &chunk_map,
                    &chunk_query,
                    &tile_query
                ) {
                    *action = new_action.clone();
                    *timer = duration.generate_timer(&action);
                    **busy = true;
                    return;
                }
            }
        }
        action.kind = PLAYER_ACTION_DEFAULT.kind;
        
    }
}

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
    let position = &player_tile_pos(&transform, offset);
    let position_relative = relative_tile_pos(&position);

    let target_pos = *position + action.get_raw_transformation();
    let target_chunk_pos = tile_pos_to_chunk_pos(&target_pos);
    
    let current_chunk_pos = tile_pos_to_chunk_pos(position);
    let (layer, _) = chunk_map.get(&current_chunk_pos).unwrap();
    let tile_storage = chunk_query.get(*layer).unwrap();
    let tile_entiy = tile_storage.get(&position_relative).unwrap();
    let level0 = **tile_query.get(tile_entiy).unwrap() as f32;

    if let Some((layer, _)) = chunk_map.get(&target_chunk_pos) {
        let tile_storage = chunk_query.get(*layer).unwrap();
        let target_relative = relative_tile_pos(&target_pos);
        if let Some(tile_entiy) = tile_storage.get(&target_relative) {
            let level1 = **tile_query.get(tile_entiy).unwrap() as f32;
            return level0 == level1;
        } else {
            return true;
        }
    }
    return false;
}

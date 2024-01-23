use bevy::ecs::component::Component;
use phf::Map;

/// Map the size of the dialogue depeding on the entered characters
#[derive(Component)]
pub struct DialogueSizePHF(pub Map<u8, usize>);

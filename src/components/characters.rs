use bevy::ecs::component::Component;


#[derive(Component)]
pub struct Health(pub u8);

#[derive(Component)]
pub struct Busy(pub bool);

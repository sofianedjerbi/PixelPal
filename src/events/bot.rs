use bevy::prelude::*;

use crate::components::gpt::GPTAgent;


#[derive(Event)]
pub struct ValidResponseEvent(GPTAgent);
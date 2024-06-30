use collisions::collisions;

use crate::prelude::*;
mod player_input;
mod map_renders;
mod entity_render;
mod collisions;
pub fn build_scheduler()->Schedule{
    Schedule::builder()
    .add_system(player_input::player_input_system())
    .add_system(collisions::collisions_system())
    .add_system(map_renders::map_render_system())
    .add_system(entity_render::entity_render_system())
    .build()
}
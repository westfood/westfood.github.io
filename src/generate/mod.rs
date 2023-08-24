use antropocene::*;
use bevy::prelude::*;
use map::generate_terrain;

mod antropocene;
mod map;

pub struct GeneratePlugin;

impl Plugin for GeneratePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems((generate_terrain, gods_created, apply_system_buffers, industry_created).chain());
    }
}

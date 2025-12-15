use antropocene::*;
use bevy::prelude::*;
use map::generate_terrain;

mod antropocene;
mod map;

pub struct GeneratePlugin;

impl Plugin for GeneratePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (generate_terrain, gods_created, industry_created).chain(),
        );
    }
}

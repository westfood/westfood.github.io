use antropocene::world_created;
use bevy::prelude::*;
use terrain::generate_terrain;

mod antropocene;
mod terrain;

pub struct TerraformPlugin;

impl Plugin for TerraformPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems((generate_terrain, world_created).chain());
    }
}

use crate::game::resources::AppState;
use bevy::prelude::*;
use systems::*;
mod systems;

pub struct GovernPlugin;

impl Plugin for GovernPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (govern, camera_control).run_if(in_state(AppState::Govern)));
    }
}

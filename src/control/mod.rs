use crate::game::resources::AppState;
use bevy::prelude::*;
use systems::*;
mod systems;

pub struct GovernPlugin;

impl Plugin for GovernPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(govern.in_set(OnUpdate(AppState::Govern)))
            .add_system(camera_control.in_set(OnUpdate(AppState::Govern)));
    }
}

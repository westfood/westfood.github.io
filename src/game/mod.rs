use crate::control::GovernPlugin;
use bevy::prelude::*;
use resources::*;
use systems::*;

pub mod resources;
mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems(
            (intro, apply_system_buffers, game_configured, spawn_camera).chain(),
        )
        .add_system(compute_yealds.in_schedule(OnExit(AppState::Govern)))
        .add_system(round_system.in_schedule(OnEnter(AppState::RoundEnd)))
        .add_system(turn_end.in_schedule(OnEnter(AppState::TurnEnd)));
    }
}

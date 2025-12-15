use crate::control::GovernPlugin;
use bevy::prelude::*;
use resources::*;
use systems::*;

pub mod resources;
mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup,
            (intro, game_configured, spawn_camera).chain(),
        )
        .add_systems(OnExit(AppState::Govern), compute_yealds)
        .add_systems(OnEnter(AppState::RoundEnd),round_system)
        .add_systems(OnEnter(AppState::TurnEnd),turn_end);
    }
}

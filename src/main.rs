#![allow(unused)]
// use std::{thread::current, error::Report};

use bevy::prelude::*;
use control::systems::*;
use game::resources::*;
use game::systems::*;
use std::iter::Filter;
use world::systems::*;

mod control;
mod game;
pub mod gods;
mod industry;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hry na civilizaci".into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameState>()
        .add_state::<AppState>()
        .add_startup_systems(
            (
                intro,
                world_created,
                apply_system_buffers,
                game_configured,
                generate_terrain,
                spawn_camera,
            )
                .chain(),
        )
        .add_system(govern.in_set(OnUpdate(AppState::Govern)))
        .add_system(camera_dragging.in_set(OnUpdate(AppState::Govern)))
        .add_system(compute_yealds.in_schedule(OnExit(AppState::Govern)))
        .add_system(round_system.in_schedule(OnEnter(AppState::RoundEnd)))
        .add_system(turn_end.in_schedule(OnEnter(AppState::TurnEnd)))
        .run();
}

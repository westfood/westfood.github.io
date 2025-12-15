#![allow(unused)]
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use control::GovernPlugin;
use game::resources::*;
use game::GamePlugin;
use generate::GeneratePlugin;
use ui::MenuControl;

mod control;
mod game;
mod generate;
mod gods;
mod industry;
mod terrain;
mod ui;

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
        .add_plugins((
            GeneratePlugin,
            GamePlugin,
            GovernPlugin,
            MenuControl,
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        ))
        .run();
}

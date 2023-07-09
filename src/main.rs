#![allow(unused)]
use bevy::prelude::*;
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
        .add_plugin(GeneratePlugin)
        .add_plugin(GamePlugin)
        .add_plugin(GovernPlugin)
        .add_plugin(MenuControl)
        .run();
}

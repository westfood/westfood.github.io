use super::super::components::GodOverview;
use bevy::prelude::*;

pub fn build_gods_overview(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let gods_overview_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(50.), Val::Percent(30.)),
                    ..default()
                },
                background_color: Color::RED.into(),
                ..default()
            },
            GodOverview {},
        ))
        .id();

    gods_overview_entity
}

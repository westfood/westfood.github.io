use super::super::components::GodOverview;
use super::super::styles::*;
// use crate::ui::styles::*;
use bevy::prelude::*;

pub fn build_gods_overview(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let gods_overview_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    // size: Size::new(Val::Px(500.), Val::Px(200.)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..default()
            },
            GodOverview {},
        ))
        .id();
    let god_overview_entity = commands
        .spawn(NodeBundle {
            style: Style {
                // size: Size::new(Val::Percent(50.), Val::Percent(50.)),
                ..default()
            },
            background_color: Color::GRAY.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Skřet",
                        get_title_text_style(&asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        })
        .set_parent(gods_overview_entity)
        .id();

    gods_overview_entity
}

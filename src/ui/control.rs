use super::components::*;
use super::gods::layout::*;

use bevy::prelude::*;
pub fn menu_control(
    keyboard_input: Res<Input<KeyCode>>,
    gods_overview_query: Query<Entity, With<GodOverview>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::G) && gods_overview_query.is_empty() {
        let gods_overview_entity = build_gods_overview(&mut commands, &asset_server);
    } else if keyboard_input.just_pressed(KeyCode::G) {
        if let Ok(gods_overview_entity) = gods_overview_query.get_single() {
            commands.entity(gods_overview_entity).despawn_recursive();
        }
    }
}

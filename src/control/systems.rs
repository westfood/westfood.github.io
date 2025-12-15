use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::gods::components::*;
use crate::industry::components::*;
use crate::terrain::components::*;
use crate::{game::resources::*, industry::systems::report_yeald};

use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    reflect::Tuple,
    sprite::queue_material2d_meshes,
    window::{self, CursorGrabMode, PrimaryWindow},
};

pub fn govern(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<GameState>,
    // terrain: Query<Entity, &Grassland>,
    grassland: Query<&Grassland>,
    desert: Query<&Desert>,
    tundra: Query<&Tundra>,
    snow: Query<&Snow>,
    query: Query<&Yield>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    // TODO Implement also Numeric Enter
    if keyboard_input.just_released(KeyCode::Return) {
        info!(
            "Bůh {} skončil své panování v tahu {}.",
            game.governing_god, game.turn
        );
        next_state.set(AppState::RoundEnd)
    }
    if keyboard_input.just_released(KeyCode::Space) {
        report_yeald(query);
    }
    if keyboard_input.just_released(KeyCode::T) {
        // grassland.for_each(|title| println!("{:?}", title));
        let grassland: usize = grassland.iter().count();
        let desert: usize = desert.iter().count();
        let tundra: usize = tundra.iter().count();
        let snow: usize = snow.iter().count();
        println!(
            "Grassland: \t{grassland}\nDesert: \t{desert}\nTundra: \t{tundra}\nSnow: \t\t{snow}"
        );
    }
}

pub fn camera_control(
    mut windows: Query<&mut Window>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_wheel: EventReader<MouseWheel>,
    mouse: Res<Input<MouseButton>>,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut projection: Query<&mut OrthographicProjection, With<Camera>>,
) {
    let mut window = windows.single_mut();
    let mut projection = projection.single_mut();

    if mouse.pressed(MouseButton::Left) {
        window.cursor.visible = false;
        for ev in ev_motion.read() {
            let mut camera = camera.single_mut();
            camera.translation += Vec3::new(
                -ev.delta.x * projection.scale,
                ev.delta.y * projection.scale,
                0.0,
            );
        }
        let current_pos = match window.cursor_position() {
            Some(current_pos) => current_pos,
            None => return,
        };
    }

    if mouse.just_released(MouseButton::Left) {
        window.cursor.visible = true;
    }
    let mut zoom_amount = projection.scale;
    for event in ev_wheel.read() {
        zoom_amount -= event.y.signum()
            * match event.unit {
                MouseScrollUnit::Line => 0.25,
                MouseScrollUnit::Pixel => 0.1,
            };
        projection.scale = zoom_amount.clamp(1.0, 10.0);
    }
}

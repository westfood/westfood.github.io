use bevy::prelude::*;

use crate::game::resources::*;
use crate::gods::components::*;
use crate::industry::components::*;

use bevy::{
    input::mouse::MouseMotion,
    pbr::RenderLightSystems,
    prelude::*,
    reflect::Tuple,
    sprite::queue_material2d_meshes,
    window::{self, CursorGrabMode, PrimaryWindow},
};

pub fn govern(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<GameState>,
    iron_yeald: Query<(&IronYeald)>,
    copper_yeald: Query<&CopperYeald>,
    tin_yeald: Query<&TinYeald>,
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
        let copper_sum: u32 = copper_yeald.iter().map(|yeald| yeald.value).sum();
        let tin_sum: u32 = tin_yeald.iter().map(|yeald| yeald.value).sum();
        let iron_sum: u32 = iron_yeald.iter().map(|yeald| yeald.value).sum();
        println!("Plánová težba bronzu: {copper_sum}, cínu: {tin_sum}, železa: {iron_sum}");
    }
}

pub fn camera_dragging(
    mut windows: Query<&mut Window>,
    mut ev_motion: EventReader<MouseMotion>,
    mouse: Res<Input<MouseButton>>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    let mut window = windows.single_mut();

    if mouse.pressed(MouseButton::Left) {
        window.cursor.visible = false;
        for ev in ev_motion.iter() {
            let mut camera = camera.single_mut();
            camera.translation += Vec3::new(-ev.delta.x, ev.delta.y, 0.0);
        }
    }

    if mouse.just_released(MouseButton::Left) {
        window.cursor.visible = true;
    }
}

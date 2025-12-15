use bevy::prelude::*;
mod components;
mod control;
mod gods;
mod styles;
use control::menu_control;

pub struct MenuControl;

impl Plugin for MenuControl {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, menu_control);
    }
}

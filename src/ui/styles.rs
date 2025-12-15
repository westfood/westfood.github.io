use bevy::prelude::*;

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/Fira_Mono/FiraMono-Medium.ttf"),
        font_size: 18.,
        color: Color::WHITE,
    }
}

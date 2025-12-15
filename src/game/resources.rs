use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct GameState {
    pub turn: u32,
    pub governing_god: String,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Govern,
    TurnEnd,
    RoundEnd,
    GameEnd,
}

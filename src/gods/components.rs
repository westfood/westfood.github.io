use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct God {
    pub name: String,
    pub alive: bool,
}

impl God {
    pub fn name(&self) -> String {
        String::from(&self.name)
    }
}

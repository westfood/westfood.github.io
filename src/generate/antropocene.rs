use crate::gods::components::*;
use crate::industry::components::*;
use crate::industry::systems::*;
use bevy::prelude::*;

pub fn gods_created(mut commands: Commands) {
    commands.spawn_batch(vec![
        God {
            name: "Hráč".to_string(),
            alive: true,
        },
        God {
            name: "Stroj".to_string(),
            alive: true,
        },
        God {
            name: "Příroda".to_string(),
            alive: false,
        },
        God {
            name: "Skřet".to_string(),
            alive: true,
        },
        God {
            name: "Skřítek".to_string(),
            alive: true,
        },
    ]);
    info!("Bohové stvořeni!");
}

pub fn industry_created(mut commands: Commands) {
    create_industry(&mut commands, &MineTypes::Copper, 5);
    create_industry(&mut commands, &MineTypes::Tin, 3);
    create_industry(&mut commands, &MineTypes::Iron, 3);
}

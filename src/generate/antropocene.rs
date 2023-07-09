use crate::gods::components::*;
use crate::industry::components::*;
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

pub fn industry_created(mut commands: Commands, gods: Query<&God>) {
    println!("{gods:#?}");
    gods.for_each(|god| println! {"{god:#?}"});
    commands.spawn((Mine::new(MineTypes::Copper), CopperYeald { value: 3 }));
    commands.spawn(Furnace::new());
    commands.spawn(Forge::new());
    commands.spawn(Foundry::new());
    // commands.spawn((Mine::new("Copper"), IronYeald{value: 3}));
    // commands.spawn((Mine::new("Tin"), IronYeald{value: 1}));
    // commands.spawn((Mine{ name: "Iron Mine".to_string()}, IronYeald{value: 3}, IronMaxCapacity{value: 10}));
    // commands.spawn((Mine{ name: "Iron Mine".to_string()}, IronYeald{value: 2}, IronMaxCapacity{value: 10}));
    // commands.spawn((Mine{ name: "Tin Yeald".to_string()}, TinYeald{value: 1}, TinMaxCapacity{value: 10}));
    // commands.spawn((Mine{ name: "Copper Yeald".to_string()}, CopperYeald{value: 2}));
    // commands.spawn((Storage{ name: "Storage".to_string()}, CopperMaxCapacity{value: 50}));
    info!("Průmysl stvořen!");
}

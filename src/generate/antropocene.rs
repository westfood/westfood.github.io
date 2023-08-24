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
    gods.for_each(|god| {
        commands.spawn((Mine::new(MineTypes::Copper, 3 ,god.name())));
        commands.spawn((Storage::new(god.name())));
    }
    );
    commands.spawn(Furnace::new());
    commands.spawn(Forge::new());
    commands.spawn(Foundry::new());
    info!("Průmysl stvořen!");
}

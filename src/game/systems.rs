use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::resources::*;
use crate::gods::components::*;
use crate::industry::components::*;

pub fn world_created(mut commands: Commands, gods: Query<&God>) {
    commands.spawn_batch(vec![
        God {
            name: "Rudolf".to_string(),
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
    info!("Antropocén vytvořen!");
}

pub fn intro() {
    println!("Budiž ti je svěřená civilizace plátnem tvojí duše.");
}

pub fn game_configured(gods: Query<&God>, mut game_state: ResMut<GameState>) {
    game_state.turn = 0;
    println!("Počet bohů: {}", gods.iter().len());
    let mut god = gods.iter().filter(|god| god.alive == true).next();
    match god {
        Some(god) => {
            game_state.governing_god = god.name();
            println!(
                "Tah {}, Začíná bůh: {}",
                game_state.turn, game_state.governing_god
            );
        }
        None => println!("Už neexistuje žádný bůh."),
    }
    info!("Pravidla Antropocénu stvořena!");
}

pub fn compute_yealds(
    mut next_state: ResMut<NextState<AppState>>,
    iron_yeald: Query<&IronYeald>,
    copper_yeald: Query<&CopperYeald>,
    tin_yeald: Query<&TinYeald>,
) {
    let copper_sum: u32 = copper_yeald.iter().map(|yeald| yeald.value).sum();
    let tin_sum: u32 = tin_yeald.iter().map(|yeald| yeald.value).sum();
    let iron_sum: u32 = iron_yeald.iter().map(|yeald| yeald.value).sum();
    println!("Vytěženo mědi: {copper_sum}, cínu: {tin_sum}, železa: {iron_sum}");
    info!("Vzniklé bohatství sečteno!");
    next_state.set(AppState::Govern);
}

pub fn turn_end(mut next_state: ResMut<NextState<AppState>>, mut game: ResMut<GameState>) {
    info!("Všichni živí bozi v tomto tahu už vládli.");
    info!("Konec tahu {}", game.turn);
    game.turn += 1;
    info!("Začíná tah {}!", game.turn);
    println!("Bůh {} se ujímá vlády.", game.governing_god);
    next_state.set(AppState::Govern);
}

pub fn round_system(
    gods: Query<(&God)>,
    mut game: ResMut<GameState>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    info!("Systém střídání bohů začíná!");
    info!(
        "Bůh {} skončil své panování v tahu {}. A předává vládu dalšímu živému bohu.",
        game.governing_god, game.turn
    );
    let mut index = gods
        .iter()
        .filter(|god| god.alive == true)
        .position(|god| god.name == game.governing_god);
    match index {
        Some(index) => {
            let mut god = gods.iter().filter(|god| god.alive == true).nth(index + 1);
            match god {
                Some(god) => {
                    game.governing_god = god.name();
                    println!("Bůh {} se ujímá vlády.", game.governing_god);
                    next_state.set(AppState::Govern);
                }
                None => {
                    let mut god = gods.iter().filter(|god| god.alive == true).next();
                    match god {
                        Some(god) => {
                            game.governing_god = god.name();
                            next_state.set(AppState::TurnEnd);
                        }
                        None => {
                            println!("Antropocén je bez bohů! Svět končí..");
                            next_state.set(AppState::GameEnd);
                        }
                    }
                }
            }
        }
        None => println!("Anything"),
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        // transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

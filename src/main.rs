#![allow(unused)]
// use std::{thread::current, error::Report};

use std::iter::Filter;

use bevy::{prelude::*, sprite::queue_material2d_meshes};
// use bevy_ecs::schedule::Schedules;
// use bevy::input::ButtonState;
// use bevy::render::extract_resource::ExtractResource;

#[derive(Resource, Debug, Default)]
struct GameState {
    turn: u8,
    round: u8,
    governing_god: String,

}
#[derive(Component, Debug, Default)]
struct God{
    name: String,
    alive: bool,
}

impl God {
    fn name(&self) -> String {
        String::from(&self.name)
    }
}

#[derive(Component)]
struct Iron(u8);

#[derive(Component, Debug)]
struct IronYeald {
    value: u8,
}

#[derive(Component, Debug)]
struct CopperYeald(u8);

#[derive(Component, Debug)]
struct TinYeald(u8);

#[derive(Component, Debug)]
struct Mine {
    name: String,

}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Govern,
    TurnEnd,
    RoundEnd,
    GameEnd,
}

fn intro() {
    println!("Budiž ti je svěřená civilizace plátnem tvojí duše.");
}

fn world_created(mut commands: Commands, gods: Query<&God>) {
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
        }
    ]);
    commands.spawn((Mine{ name: "Iron Mine".to_string()}, IronYeald{ value: 2 }));
    commands.spawn((Mine{ name: "Iron Mine".to_string()}, IronYeald{ value: 1 }));
    info!("Antropocén vytvořen!");
}

fn game_configured(gods: Query<&God>, mut game_state: ResMut<GameState>) {
    game_state.turn = 0;
    println!("Počet bohů: {}", gods.iter().len());
    let mut god = gods.iter().filter(|god| god.alive == true ).next();
    match god {
        Some(god) => {
            game_state.governing_god = god.name();
            println!("Tah {}, Začíná bůh: {}", game_state.turn, game_state.governing_god);
        },
        None => println!("Už neexistuje žádný bůh."),
    }
    info!("Pravidla Antropocénu stvořena!");
}

fn compute_yealds(mut next_state: ResMut<NextState<AppState>>, iron_yeald: Query<(&IronYeald)>) {
    info!("Civilizace za tah získala!");
    next_state.set(AppState::Govern);
}

fn turn_end (
    mut next_state: ResMut<NextState<AppState>>,
    mut game: ResMut<GameState>,
)
    {
        info!("Všichni živí bozi v tomto tahu už vládli.");
        info!("Konec tahu {}", game.turn);
        game.turn += 1;
        info!("Začíná tah {}!", game.turn);
        println!("Bůh {} se ujímá vlády.", game.governing_god);  
        next_state.set(AppState::Govern);
}

fn round_system(
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
                },
                None => {
                    let mut god = gods.iter().filter(|god| god.alive == true).next();
                    match god {
                        Some(god) => { 
                            game.governing_god = god.name();
                            next_state.set(AppState::TurnEnd);
                        },
                        None => { 
                            println!("Antropocén je bez bohů! Svět končí..");
                            next_state.set(AppState::GameEnd);
                        }
                }
            }
        }
    },
    None => println!("Anything")
}
}

fn govern(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    iron_yeald: Query<(&IronYeald)>,
    iron: Query<(&Iron)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    // TODO Implement also Numeric Enter
    if keyboard_input.just_released(KeyCode::Return) {
        info!(
            "Bůh {} skončil své panování v kole {}.",
            game_state.governing_god, game_state.round
        );
        next_state.set(AppState::RoundEnd)
    }
    if keyboard_input.just_released(KeyCode::Space) {
        for yeald in &iron_yeald {
            let sum: u8 = yeald.value;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hry na civilizaci".into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameState>()
        .add_state::<AppState>()
        .add_startup_systems((intro, world_created, apply_system_buffers, game_configured).chain())
        .add_system(govern.in_set(OnUpdate(AppState::Govern)))
        .add_system(compute_yealds.in_schedule(OnExit(AppState::Govern)))
        .add_system(round_system.in_schedule(OnEnter(AppState::RoundEnd)))
        .add_system(turn_end.in_schedule(OnEnter(AppState::TurnEnd)))
        .run();
}

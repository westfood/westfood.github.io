#![allow(unused)]
// use std::{thread::current, error::Report};

use std::iter::Filter;

use bevy::{prelude::*, sprite::queue_material2d_meshes};
// use bevy_ecs::schedule::Schedules;
// use bevy::input::ButtonState;
// use bevy::render::extract_resource::ExtractResource;

#[derive(Resource, Debug, Default)]
struct GameState {
    turn: u32,
    governing_god: String,

}
#[derive(Component, Debug, Default)]
struct God {
    name: String,
    alive: bool,
}

impl God {
    fn name(&self) -> String {
        String::from(&self.name)
    }
}

#[derive(Component, Debug)]
struct IronYeald {
    value: u32
}

#[derive(Component, Debug)]
struct CopperYeald {
    value: u32
}

#[derive(Component, Debug)]
struct TinYeald {
    value: u32
}


enum MineTypes {
    Iron,
    Tin,
    Copper,
}


impl MineTypes {
    fn is_valid(&self) -> bool {
        match self {
            MineTypes::Iron | MineTypes::Tin | MineTypes::Copper  => true,
            _ => false
        }
    }
    fn name(&self) -> String {
        match self {
            MineTypes::Iron => String::from("Iron Mine"),
            MineTypes::Copper => String::from("Copper Mine"),
            MineTypes::Tin => String::from("Tin Mine"),
        }
    }
}

#[derive(Component, Debug)]
struct Mine {
    name: String
}

impl Mine {
    fn new(mine_type: MineTypes) -> Mine{
        if mine_type.is_valid() { 
            info!("Spawning {}", mine_type.name());
        {
            Mine {
            name: mine_type.name()
            }
        }} else { 
            panic!("Wrong Mine Type")
        }
    }
}

#[derive(Component, Debug)]
struct Storage {
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
    commands.spawn((Mine::new(MineTypes::Iron), IronYeald{value: 3}));
    // commands.spawn((Mine::new("Copper"), IronYeald{value: 3}));
    // commands.spawn((Mine::new("Tin"), IronYeald{value: 1}));
    // commands.spawn((Mine{ name: "Iron Mine".to_string()}, IronYeald{value: 3}, IronMaxCapacity{value: 10}));
    // commands.spawn((Mine{ name: "Iron Mine".to_string()}, IronYeald{value: 2}, IronMaxCapacity{value: 10}));
    // commands.spawn((Mine{ name: "Tin Yeald".to_string()}, TinYeald{value: 1}, TinMaxCapacity{value: 10}));
    // commands.spawn((Mine{ name: "Copper Yeald".to_string()}, CopperYeald{value: 2}));
    // commands.spawn((Storage{ name: "Storage".to_string()}, CopperMaxCapacity{value: 50}));
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

fn compute_yealds(
    mut next_state: ResMut<NextState<AppState>>,
    iron_yeald: Query<&IronYeald>,
    copper_yeald: Query<&CopperYeald>,
    tin_yeald: Query<&TinYeald>,
) {
    let copper_sum: u32 = copper_yeald.iter().map(|yeald| yeald.value).sum();
    let tin_sum: u32 = tin_yeald.iter().map(|yeald| yeald.value).sum();
    let iron_sum: u32 = iron_yeald.iter().map(|yeald| yeald.value).sum();
    println!("Vytěženo bronzu: {copper_sum}, cínu: {tin_sum}, železa: {iron_sum}");
    info!("Vzniklé bohatství sečteno!");
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

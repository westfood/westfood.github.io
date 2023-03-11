#![allow(unused)]
// use std::{thread::current, error::Report};

use bevy::prelude::*;
// use bevy::input::ButtonState;
// use bevy::render::extract_resource::ExtractResource;

#[derive(Resource, Default)]
struct GameState {
    turn: u8,
}

#[derive(Component, Debug)]
struct Player(String);

#[derive(Component, Debug)]
struct Building {
    name: String,
    owner: Player,
    yeald: Yeald,
}
#[derive(Component, Debug)]
struct Yeald {
    iron: (u8),
    tin: (u8),
    copper: (u8),
}
#[derive(Component, Debug)]
struct Resources {
    iron: (u8),
    tin: (u8),
    copper: (u8),
}

#[derive(Component)]
struct Storage {
    stored: Resources,
}


fn intro() {
    println!("Budiž ti je svěřená civilizace plátnem tvojí duše.");
}

fn govern(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    buildings: Query<(&Building)>,
    resources: Query<(&Resources)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    // TODO Implement also Numeric Enter
    if keyboard_input.just_released(KeyCode::Return) {
        info!(
            "Tah {} končí.",
            game_state.turn
        );
        game_state.turn += 1;
        info!(
            "Tah {} začíná.",
            game_state.turn);
        next_state.set(AppState::TurnEnd)
    }
    if keyboard_input.just_released(KeyCode::Space) {
        for building in &buildings {
            eprintln!("Jméno Budovy: {}, Produkce: Železo {}, Cín {}, Měď {}", building.name, building.yeald.iron, building.yeald.tin, building.yeald.copper);
        };
        for (resource) in resources.iter() {
            eprintln!("Záasoby: Železo {}, Cín {}, Měď {}", resource.iron, resource.tin, resource.copper)
        };
    }
}

fn turn_report (
    mut next_state: ResMut<NextState<AppState>>,
    query: Query<(&Resources)>
)
    {
        info!("Report");
        for resource in &query {
            eprintln!("Zdroj: {}", resource.iron)
        }
        next_state.set(AppState::InGame);
}

fn initial_game_state(mut commands: Commands, mut game_state: ResMut<GameState>, state: Res<State<AppState>>) {
    game_state.turn = 0;
    println!(
        "Tah {} začíná.",
        game_state.turn
    );
    commands.spawn(Building {name: "IronMine".to_string(), owner: Player("Player0".to_string()), yeald: Yeald{iron: 2, tin: 0, copper: 0}});
    commands.spawn(Building {name: "CooperMine".to_string(), owner: Player("Player0".to_string()), yeald: Yeald{iron: 0, tin: 0, copper: 2}});
    commands.spawn(Building {name: "TinMine".to_string(), owner: Player("Player0".to_string()), yeald: Yeald{iron: 0, tin: 2, copper: 0}});
    commands.spawn(Storage {stored: Resources{iron: 5, tin: 1, copper: 3}});
    commands.spawn(Storage {stored: Resources{iron: 10, tin: 2, copper: 6}});
    // commands.spawn((Resources{iron: 10, tin: 10, copper: 0}));
    println!("První budovy osídleny!");
}

fn compute_yealds(mut next_state: ResMut<NextState<AppState>>, buildings: Query<(&Building)>, mut resources: Query<(&mut Resources)>) {
    info!("Compute Yealds");
    for (mut resource) in &mut resources {
        for building in &buildings {
            resource.iron += building.yeald.iron;
            resource.tin += building.yeald.tin;
            resource.copper += building.yeald.copper;
        };
    };
    println!("query result: {:#?}",buildings);
    for building in buildings.iter() {
        println!("iterated result: {:#?}", building)
    };
    next_state.set(AppState::InGame);
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    InGame,
    TurnEnd,
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
        .add_state::<AppState>()
        .init_resource::<GameState>()
        .add_systems(((intro, initial_game_state).chain()).on_startup())
        .add_system(govern.in_set(OnUpdate(AppState::InGame)))
        .add_system(compute_yealds.in_schedule(OnExit(AppState::InGame)))
        .add_system(turn_report.in_set(OnUpdate(AppState::TurnEnd)))
        .run();
}

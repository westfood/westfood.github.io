#![allow(unused)]
// use std::{thread::current, error::Report};

use std::iter::Filter;

use bevy::{
    input::mouse::MouseMotion,
    pbr::RenderLightSystems,
    prelude::*,
    reflect::Tuple,
    sprite::queue_material2d_meshes,
    window::{self, CursorGrabMode, PrimaryWindow},
};

use rand::{distributions::WeightedIndex, prelude::*};
// use bevy_ecs::schedule::Schedules;
// use bevy::input::ButtonState;
// use bevy::render::extract_resource::ExtractResource;

const MAP_HEIGHT: u32 = 100;
const MAP_WIDTH: u32 = 100;
// Image HEIGHT is 140 - but hexagons are placed with offset
const TILE_HEIGHT: f32 = 104.0;
const TILE_WIDTH: f32 = 120.0;
// Every second vertical tile need to be offseted by half of tile width.
const TILE_OFFSET: f32 = 60.0;

fn generate_terrain(asset_server: Res<AssetServer>, mut commands: Commands) {
    for x in 0..MAP_HEIGHT {
        for y in 0..MAP_WIDTH {
            let mut tile_x: f32;

            // Every second vertical tile have to be offseted by half of tile width.
            if y % 2 == 0 {
                let x: f32 = x as f32;
                tile_x = x * TILE_WIDTH - TILE_OFFSET;
            } else {
                let x: f32 = x as f32;
                tile_x = x * TILE_WIDTH;
            }

            let mut rng = rand::thread_rng();

            // Grasslands
            let grass_assets = [5, 10, 11, 12, 13, 14, 15, 16];
            let grass_weights = [7.0, 3.0, 3.0, 3.0, 3.0, 0.1, 0.1, 0.1];
            let grass_dist = WeightedIndex::new(&grass_weights).unwrap();
            let grass_random_index = grass_dist.sample(&mut rng);
            let grass_id = format!(
                "hexagon-pack/PNG/Tiles/Terrain/Grass/grass_{:02}.png",
                grass_assets[grass_random_index]
            );

            // Deserts
            let sand_assets = [7, 12, 13, 14, 15, 16, 17, 18];
            let sand_weights = [7.0, 3.0, 3.0, 3.0, 3.0, 0.1, 0.1, 0.1];
            let sand_dist = WeightedIndex::new(&sand_weights).unwrap();
            let sand_random_index = sand_dist.sample(&mut rng);
            let sand_id = format!(
                "hexagon-pack/PNG/Tiles/Terrain/Sand/sand_{:02}.png",
                sand_assets[sand_random_index]
            );

            // Tundra
            let tundra_assets = [6, 11, 12, 13, 14, 15, 16, 17, 18];
            let tundra_weights = [7.0, 3.0, 3.0, 3.0, 3.0, 0.1, 0.1, 0.1, 0.1];
            let tundra_dist = WeightedIndex::new(&tundra_weights).unwrap();
            let tundra_random_index = tundra_dist.sample(&mut rng);
            let tundra_id = format!(
                "hexagon-pack/PNG/Tiles/Terrain/Dirt/dirt_{:02}.png",
                tundra_assets[sand_random_index]
            );

            // Snow
            // let sand_assets = [7, 12, 13, 14, 15, 16, 17, 18];
            // let sand_weights = [7.0, 3.0, 3.0, 3.0, 3.0, 0.1, 0.1, 0.1];
            // let sand_dist = WeightedIndex::new(&grass_weights).unwrap();
            // let sand_random_index = sand_dist.sample(&mut rng);
            // let sand_id = format!(
            //     "hexagon-pack/PNG/Tiles/Terrain/Stone/sand{:02}.png",
            //     sand_assets[sand_random_index]
            // );

            let y: f32 = y as f32;
            let tile_y: f32 = y * TILE_HEIGHT;
            let y_region = y as u32;

            if y_region <= 3 || y_region >= MAP_HEIGHT - 3 {
                // SNOW
                commands.spawn(
                    (SpriteBundle {
                        transform: Transform::from_xyz(tile_x, tile_y, 0.0),
                        texture: asset_server
                            .load("hexagon-pack/PNG/Tiles/Terrain/Stone/stone_07.png"),
                        ..default()
                    }),
                );
            } else if (y_region >= 4 && y_region <= 15)
                || (y_region >= MAP_HEIGHT - 15 && y_region <= MAP_HEIGHT - 4)
            {
                // TUNDRA
                commands.spawn(
                    (SpriteBundle {
                        transform: Transform::from_xyz(tile_x, tile_y, 0.0),
                        texture: asset_server.load(tundra_id),
                        ..default()
                    }),
                );
            } else if y_region >= MAP_HEIGHT / 2 - 5 && y_region <= MAP_HEIGHT / 2 + 5 {
                // DESERT
                commands.spawn(
                    (SpriteBundle {
                        transform: Transform::from_xyz(tile_x, tile_y, 0.0),
                        texture: asset_server.load(sand_id),
                        ..default()
                    }),
                );
            } else {
                // GRASS
                commands.spawn(
                    (SpriteBundle {
                        transform: Transform::from_xyz(tile_x, tile_y, 0.0),
                        texture: asset_server.load(grass_id),
                        ..default()
                    }),
                );
            }
        }
    }
}
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
    value: u32,
}

#[derive(Component, Debug)]
struct CopperYeald {
    value: u32,
}

#[derive(Component, Debug)]
struct TinYeald {
    value: u32,
}

enum MineTypes {
    Iron,
    Tin,
    Copper,
}

impl MineTypes {
    fn is_valid(&self) -> bool {
        match self {
            MineTypes::Iron | MineTypes::Tin | MineTypes::Copper => true,
            _ => false,
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
    name: String,
    asset_id: String,
}

impl Mine {
    fn new(mine_type: MineTypes) -> Mine {
        if mine_type.is_valid() {
            info!("Spawning {}", mine_type.name());
            {
                Mine {
                    name: mine_type.name(),
                    asset_id: format!("hexagon-pack/PNG/Tiles/Medieval/medieval_mine.png"),
                }
            }
        } else {
            panic!("Wrong Mine Type")
        }
    }
}

#[derive(Component, Debug)]
struct Forge {
    name: String,
}

impl Forge {
    fn new() -> Forge {
        Forge {
            name: "Forge".to_string(),
        }
    }
}
#[derive(Component, Debug)]
struct Furnace {
    name: String,
}

impl Furnace {
    fn new() -> Furnace {
        Furnace {
            name: "Furnace".to_string(),
        }
    }
}
#[derive(Component, Debug)]
struct Foundry {
    name: String,
}

impl Foundry {
    fn new() -> Foundry {
        Foundry {
            name: "Foundry".to_string(),
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

fn game_configured(gods: Query<&God>, mut game_state: ResMut<GameState>) {
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

fn compute_yealds(
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

fn turn_end(mut next_state: ResMut<NextState<AppState>>, mut game: ResMut<GameState>) {
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

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        // transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

fn camera_dragging(
    mut windows: Query<&mut Window>,
    mut ev_motion: EventReader<MouseMotion>,
    mouse: Res<Input<MouseButton>>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    let mut window = windows.single_mut();

    if mouse.pressed(MouseButton::Left) {
        window.cursor.visible = false;
        for ev in ev_motion.iter() {
            let mut camera = camera.single_mut();
            camera.translation += Vec3::new(-ev.delta.x, ev.delta.y, 0.0);
        }
    }

    if mouse.just_released(MouseButton::Left) {
        window.cursor.visible = true;
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
        .add_startup_systems(
            (
                intro,
                world_created,
                apply_system_buffers,
                game_configured,
                generate_terrain,
                spawn_camera,
            )
                .chain(),
        )
        .add_system(govern.in_set(OnUpdate(AppState::Govern)))
        .add_system(camera_dragging.in_set(OnUpdate(AppState::Govern)))
        .add_system(compute_yealds.in_schedule(OnExit(AppState::Govern)))
        .add_system(round_system.in_schedule(OnEnter(AppState::RoundEnd)))
        .add_system(turn_end.in_schedule(OnEnter(AppState::TurnEnd)))
        .run();
}

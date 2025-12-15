use bevy::prelude::*;

use crate::terrain::components::*;
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

pub fn generate_terrain(asset_server: Res<AssetServer>, mut commands: Commands) {
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
            let snow_assets = [1, 2, 3, 4, 5];
            let snow_weights = [7., 3., 3., 0.1, 0.1];
            let snow_dist = WeightedIndex::new(&snow_weights).unwrap();
            let snow_random_index = snow_dist.sample(&mut rng);
            let snow_id = format!(
                "hexagon-holiday-pack/PNG/Hexagon pack/Default size/hexagonTile_{:02}.png",
                snow_assets[snow_random_index]
            );

            let y: f32 = y as f32;
            let tile_y: f32 = y * TILE_HEIGHT;
            let y_region = y as u32;

            if y_region <= 3 || y_region >= MAP_HEIGHT - 3 {
                // SNOW
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(tile_x, tile_y, 0.0),
                        texture: asset_server.load(snow_id),
                        ..default()
                    },
                    Snow {},
                    Terrain {},
                    Name::new("Snow"),
                ));
            } else if (y_region >= 4 && y_region <= 15)
                || (y_region >= MAP_HEIGHT - 15 && y_region <= MAP_HEIGHT - 4)
            {
                // TUNDRA
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(tile_x, tile_y, 0.0),
                        texture: asset_server.load(tundra_id),
                        ..default()
                    },
                    Tundra {},
                    Terrain {},
                    Name::new("Tundra"),
                ));
            } else if y_region >= MAP_HEIGHT / 2 - 5 && y_region <= MAP_HEIGHT / 2 + 5 {
                // DESERT
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(tile_x, tile_y, 0.0),
                        texture: asset_server.load(sand_id),
                        ..default()
                    },
                    Desert {},
                    Terrain {},
                    Name::new("Desert"),
                ));
            } else {
                // GRASS
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(tile_x, tile_y, 0.0),
                        texture: asset_server.load(grass_id),
                        ..default()
                    },
                    Grassland {},
                    Terrain {},
                    Name::new("Grassland"),
                ));
            }
        }
    }
}

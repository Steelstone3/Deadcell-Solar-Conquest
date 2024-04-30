use std::ops::Range;

pub const TILE_SIZE: f32 = 1024.0;
// TODO ships heal in a radius 4 times the size of the base (mechanic)
pub const STATION_SIZE: f32 = TILE_SIZE / 2.0;
pub const SPACE_STATION_DISTANCE_FROM_CENTRE: f32 = TILE_SIZE * 5.0;
pub const MAP_TILES: Range<i32> = -NUMBER_OF_TILES..NUMBER_OF_TILES;
// pub const MAP_SIZE: f32 = TILE_SIZE * NUMBER_OF_TILES as f32;
pub const NUMBER_OF_TILES: i32 = 8;

// TODO randomly size these
// TODO spawned starships take damage a radius 1.5 to 2 times the suns size (mechanic)
pub const SUN_TILE_SIZE: f32 = TILE_SIZE; // minimum size
pub const PLANET_TILE_SIZE: f32 = TILE_SIZE / 2.0; // maximum size
pub const PLANET_DISTANCE_FROM_SPACESTATION: f32 = TILE_SIZE / 1.5; // minimum distance

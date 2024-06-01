pub const TILE_SIZE: f32 = 32.0;
pub const SPACE_TILE_SIZE: f32 = 32.0 * TILE_SIZE;
// TODO ships heal in a radius 4 times the size of the base (mechanic)
pub const STATION_SIZE: f32 = SPACE_TILE_SIZE / 2.0;
pub const SPACE_STATION_DISTANCE_FROM_CENTRE: f32 = SPACE_TILE_SIZE * 5.0;
pub const NUMBER_OF_TILES: i32 = 8;

// TODO randomly size these
// TODO spawned starships take damage a radius 1.5 to 2 times the suns size (mechanic)
pub const SUN_TILE_SIZE: f32 = SPACE_TILE_SIZE; // minimum size
pub const PLANET_TILE_SIZE: f32 = SPACE_TILE_SIZE / 2.0; // maximum size
pub const PLANET_CLOSEST_DISTANCE_TO_SUN: f32 = SPACE_TILE_SIZE * 1.5; // minimum distance

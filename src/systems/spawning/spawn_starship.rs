// use crate::{
//     assets::{images::faction_starships::starships::StarshipSprite, user_interace::icons::starships::atark_icons::AtarkIcon}, components::starship::{self, Starship}, events::{
//         spawn_animated_sprite_event::SpawnAnimatedSpriteEvent, spawn_sprite_event::SpawnSpriteEvent,
//     }, queries::{
//         camera_queries::CameraTransformOrthographicProjectionQuery, window_queries::WindowQuery,
//     }, resources::{selected_item::SelectedMenuItem, spawn_menu_selection::SpawnMenuSelection}, systems::{controllers::get_location::get_cursor_location, user_interface::interactions::spawn_selection::SpawnSelection}
// };
// use bevy::{
//     ecs::{
//         event::EventWriter,
//         system::{Commands, Query, ResMut},
//     },
//     input::{mouse::MouseButton, ButtonInput},
//     transform::components::Transform,
//     utils::tracing,
// };

// pub fn spawn_animal(
//     mut commands: Commands,
//     selected_item: ResMut<SpawnMenuSelection>,
//     mut mouse_button_input: ResMut<ButtonInput<MouseButton>>,
//     mut spawn_animated_sprite_event: EventWriter<SpawnAnimatedSpriteEvent>,
//     windows_query: Query<WindowQuery>,
//     camera_queries: Query<CameraTransformOrthographicProjectionQuery>,
// ) {
//     if selected_item.animal_selection == ZooAnimal::None {
//         return;
//     }

//     let Ok(window_query) = windows_query.get_single() else {
//         return;
//     };

//     let Ok(camera_query) = camera_queries.get_single() else {
//         return;
//     };

//     if !mouse_button_input.clear_just_pressed(MouseButton::Right) {
//         return;
//     }

//     // let mut animal = Animal::new_16(selected_item.animal_selection);

//     // if selected_item.animal_selection == ZooAnimal::Gorilla
//     //     || selected_item.animal_selection == ZooAnimal::Moose
//     //     || selected_item.animal_selection == ZooAnimal::RearingNightmare
//     //     || selected_item.animal_selection == ZooAnimal::StormGiant
//     // {
//     //     animal = Animal::new_32(selected_item.animal_selection)
//     // }

//     let mut starship = Starship::new(StarshipSprite::None);

//     if selected_item.starship_selection != AtarkIcon::None {

//     }

//     if starship.starship_sprite_bundle.starship_sprite == StarshipSprite::None {
//         return
//     }

//     let transform = Transform::default();

//     if let Some(position) = window_query.window.cursor_position() {
//         get_cursor_location(&mut transform, position, window_query, camera_query);
//     } else {
//         return;
//     }

//     tracing::info!("starship at {:?}", transform.translation);

//         spawn_animated_sprite_event.send(SpawnAnimatedSpriteEvent {
//             frame_timing: animal.frame_timing,
//         frame_count: animal.frame_count,
//         tile_size: animal.tile_size,
//         tile_columns: animal.frame_count as u32,
//         spawn_sprite_event: SpawnSpriteEvent {
//             sprite_path: animal.sprite_path.to_string(),
//             size: animal.size,
//             transform,
//             entity: commands.spawn(animal).id(),
//         },
//     });
// }

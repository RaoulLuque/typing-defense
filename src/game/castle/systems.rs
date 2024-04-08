use bevy::window::PrimaryWindow;
use rand::Rng;

use crate::menu::{systems::Restart, MenuState};

use super::*;

/// Spawns the castle in the middle of the screen
pub fn spawn_castle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let castle_number = i32::to_string(&rand::thread_rng().gen_range(0..4));
    let castle_sprite_path = format!("sprites/castle/castle{}.png", castle_number);
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, window.height() * 0.05, 1.0),
            texture: asset_server.load(castle_sprite_path),
            ..default()
        },
        Castle {},
        Name::new("Castle"),
    ));
}

/// Despawns the castle if all lives are gone and spawns the destroyed castle
pub fn despawn_castle_if_all_lives_are_gone_and_spawn_destroyed_castle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut castle_query: Query<(Entity, &mut Handle<Image>), With<Castle>>,
    number_of_lives_left: Res<NumberOfLivesLeft>,
    mut next_is_lost_state: ResMut<NextState<LoosingState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
    mut menu_state_next_state: ResMut<NextState<MenuState>>,
) {
    if number_of_lives_left.number == 0 {
        if let Ok((castle_entity, mut castle_image)) = castle_query.get_single_mut() {
            commands.entity(castle_entity).remove::<Castle>();
            commands.entity(castle_entity).insert(DestroyedCastle {});
            *castle_image = asset_server.load("sprites/castle/castleDestroyed.png");
            next_is_lost_state.set(LoosingState::Lost);
            simulation_state_next_state.set(SimulationState::Paused);
            menu_state_next_state.set(MenuState::LostMenu);
        }
    }
}

/// Despawns the castle and resets lives on restart event
pub fn despawn_castle_and_reset_lives_on_restart(
    mut commands: Commands,
    mut number_of_lives_left: ResMut<NumberOfLivesLeft>,
    castle_query: Query<Entity, With<Castle>>,
    destroyed_castle_query: Query<Entity, With<DestroyedCastle>>,
    mut restart_event_reader: EventReader<Restart>,
) {
    for _ in restart_event_reader.read() {
        number_of_lives_left.number = 5;
        if let Ok(castle_entity) = castle_query.get_single() {
            commands.entity(castle_entity).despawn_recursive();
        }
        if let Ok(destroyed_castle_entity) = destroyed_castle_query.get_single() {
            commands.entity(destroyed_castle_entity).despawn_recursive();
        }
    }
}

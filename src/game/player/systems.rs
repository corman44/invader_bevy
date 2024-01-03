use bevy::{prelude::*, window::PrimaryWindow};
use bevy_kira_audio::{Audio, AudioControl};

use enemy::{components::Enemy, resources::*};
use crate::AppState;
use crate::events::GameOver;
use crate::game::{enemy, SimulationState};
use crate::game::player::PlayerState;
use crate::game::score::resources::Score;
use crate::game::star::components::Star;
use crate::game::star::resources::STAR_SIZE;
use super::components::Player;

pub const PLAYER_SIZE: f32  = 64.0;
pub const PLAYER_SPEED: f32 = 400.0;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                texture: asset_server.load("sprites/ball_blue_large.png"),
                ..default()
            },
            Player {},
        )
    );
}

pub fn player_movement (
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::new(0.0,0.0,0.0);

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radiues = ENEMY_SIZE / 2.0;

            if distance < player_radius + enemy_radiues {
                println!("Enemy Hit Player!");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);
                commands.entity(player_entity).despawn();
                commands.insert_resource(NextState(Some(PlayerState::Dead)));
                //commands.insert_resource(NextState(Some(AppState::GameOver)));
                //commands.insert_resource(NextState(Some(SimulationState::Paused)));
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    star_query: Query<(Entity,&Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score: ResMut<Score>,
) {
    if let Ok((_player_entity, player_transform)) = player_query.get_single_mut() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;

            if distance < player_radius + star_radius {
                println!("Player Hit Star!");
                let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
                audio.play(sound_effect);
                commands.entity(star_entity).despawn();
                score.value += 1;
            }
        }
    }
}

pub fn respawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        let window = window_query.get_single().unwrap();
        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                    texture: asset_server.load("sprites/ball_blue_large.png"),
                    ..default()
                },
                Player {},
            )
        );
        commands.insert_resource(NextState(Some(PlayerState::Alive)));
    }
}

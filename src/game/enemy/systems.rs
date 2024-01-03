use bevy::{prelude::*, window::PrimaryWindow};
use bevy_kira_audio::Audio;
use rand::distributions::Standard;
use rand::prelude::*;

use super::components::Enemy;
use super::resources::*;

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let rand_x = random::<f32>() * window.width();
        let rand_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn despawn_enemies(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>
) {
    for enemy in enemy_query.iter() {
        commands.entity(enemy).despawn();
    }
}

pub fn enemy_movement (
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn confine_enemy_movement (
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        let window = window_query.get_single().unwrap();

        let half_enemy_size = ENEMY_SIZE / 2.0;
        let x_min = 0.0 + half_enemy_size;
        let x_max = window.width() - half_enemy_size;
        let y_min = 0.0 + half_enemy_size;
        let y_max = window.height() - half_enemy_size;

        let mut translation = transform.translation;

        // if at edge, bounce
        if translation.x < x_min {
            translation.x = x_min;
            enemy.direction.x *= -1.0;
            // audio.play(asset_server.load("audio/impactGeneric_light_000.ogg"));
        } else if translation.x > x_max {
            translation.x = x_max;
            enemy.direction.x *= -1.0;
            // audio.play(asset_server.load("audio/impactGeneric_light_000.ogg"));
        }
        if translation.y < y_min {
            translation.y = y_min;
            enemy.direction.y *= -1.0;
            // audio.play(asset_server.load("audio/impactGeneric_light_001.ogg"));
        } else if translation.y > y_max {
            translation.y = y_max;
            enemy.direction.y *= -1.0;
            // audio.play(asset_server.load("audio/impactGeneric_light_001.ogg"));
        }
        transform.translation = translation;
    }
}

pub fn tick_enemy_spawn_timer( mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemy_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_query: Query<Entity, With<Enemy>>,
    enemy_spawn_timer: Res<EnemySpawnTimer>
) {
    if enemy_spawn_timer.timer.finished() && enemy_query.iter().len() < MAX_NUMBER_ENEMIES {
        let window = window_query.get_single().unwrap();
        let rand_x = random::<f32>() * window.height();
        let rand_y = random::<f32>() * window.width();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}



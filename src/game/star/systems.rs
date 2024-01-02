use bevy::{prelude::*, window::PrimaryWindow};

use rand::prelude::*;
use super::{resources::{NUMBER_OF_STARS, MAX_NUMBER_STARS, StarSpawnTimer}, components::Star};

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        let rand_x = random::<f32>() * window.width();
        let rand_y = random::<f32>() * window.height();

        commands.spawn( (
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {}
        ));
    }
}

pub fn tick_star_spawn_timer( mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_star_over_time( 
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
    star_query: Query<Entity, With<Star>>
) {
    if star_spawn_timer.timer.finished() && star_query.iter().len() < MAX_NUMBER_STARS {
        let window = window_query.get_single().unwrap();
        let rand_x = random::<f32>() * window.height();
        let rand_y = random::<f32>() * window.width();

        commands.spawn( (
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {}
        ));
    }
}
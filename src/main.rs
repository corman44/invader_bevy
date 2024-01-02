use bevy::{prelude::*, window::PrimaryWindow};
use bevy::app::AppExit;
use enemy::EnemyPlugin;
use events::{GameOver, exit_game, handle_game_over};
use player::PlayerPlugin;
use rand::{prelude::*, distributions::Standard};
use bevy_kira_audio::prelude::*;
use score::ScorePlugin;
use star::StarPlugin;

mod enemy;
mod player;
mod score;
mod star;
mod events;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AudioPlugin))
        .add_event::<GameOver>()
        .add_plugins((EnemyPlugin, PlayerPlugin, StarPlugin, ScorePlugin))
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (exit_game,handle_game_over))
        .run();
}


pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3) {
            0 => Direction::Left,
            1 => Direction::Right,
            2 => Direction::Up,
            _ => Direction::Down,
        }
    }
}

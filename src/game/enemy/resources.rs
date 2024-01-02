use bevy::prelude::*;

pub const ENEMY_SPAWN_TIME: f32 = 3.0;
pub const NUMBER_OF_ENEMIES: usize = 5;
pub const ENEMY_SPEED: f32 = 100.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const MAX_NUMBER_ENEMIES: usize = 15;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer { 
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
use bevy::app::{Plugin, Startup, Update};

use self::{resources::EnemySpawnTimer, systems::{spawn_enemies, enemy_movement, confine_enemy_movement, tick_enemy_spawn_timer, spawn_enemy_over_time}};

pub mod components;
pub mod resources;
mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(Startup, spawn_enemies)
            .add_systems(Update, (enemy_movement, enemy_movement, confine_enemy_movement))
            .add_systems(Update, (tick_enemy_spawn_timer, spawn_enemy_over_time));
    }
}
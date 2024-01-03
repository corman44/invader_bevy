use bevy::{app::{Plugin, Update}, ecs::schedule::{IntoSystemConfigs, common_conditions::in_state, OnEnter, OnExit}};

pub mod components;
pub mod resources;
mod systems;

use crate::AppState;
use self::{resources::EnemySpawnTimer, systems::{spawn_enemies, enemy_movement, confine_enemy_movement, tick_enemy_spawn_timer, spawn_enemy_over_time, despawn_enemies}};
use super::SimulationState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_enemies)
            .add_systems(Update, (
                enemy_movement,
                enemy_movement,
                confine_enemy_movement,
                tick_enemy_spawn_timer,
                spawn_enemy_over_time
            )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)))
            .add_systems(OnExit(AppState::Game), despawn_enemies);
    }
}
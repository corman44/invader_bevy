use bevy::prelude::*;

use crate::AppState;

use self::{systems::{spawn_stars, tick_star_spawn_timer, spawn_star_over_time, despawn_stars}, resources::StarSpawnTimer};

use super::SimulationState;

pub mod components;
pub mod resources;
mod systems;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_stars)
            .add_systems(Update, (
                tick_star_spawn_timer,
                spawn_star_over_time)
                    .run_if(in_state(SimulationState::Running))
                    .run_if(in_state(AppState::Game))
            )
            .add_systems(OnExit(AppState::Game), despawn_stars);
    }
}
use bevy::prelude::*;

use self::{systems::{spawn_stars, tick_star_spawn_timer, spawn_star_over_time}, resources::StarSpawnTimer};

pub mod components;
pub mod resources;
mod systems;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(Startup, spawn_stars)
            .add_systems(Update, (tick_star_spawn_timer,spawn_star_over_time));
    }
}
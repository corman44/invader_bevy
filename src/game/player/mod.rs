use bevy::prelude::*;

pub mod systems;
mod components;

use systems::*;

use crate::AppState;

use super::SimulationState;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<PlayerState>()
            .configure_sets(Update, MovementSystemSet.before(ConfinementSystemSet))
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (
                player_movement.in_set(MovementSystemSet),
                confine_player_movement.in_set(ConfinementSystemSet),
                enemy_hit_player,
                player_hit_star,)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))
            )
            .add_systems(Update, 
                respawn_player.run_if(in_state(PlayerState::Dead))
            );
    }
}

#[derive(Debug, States, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum PlayerState {
    #[default]
    Alive,
    Dead,
}
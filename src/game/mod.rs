pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
mod systems;

use bevy::prelude::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use crate::{events::GameOver, AppState};
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_plugins((
                EnemyPlugin,
                PlayerPlugin,
                StarPlugin,
                ScorePlugin
            ))
            .add_systems(Update, toggle_simulation
                .run_if(in_state(AppState::Game))
            );
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
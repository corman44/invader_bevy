pub mod enemy;
pub mod player;
pub mod score;
pub mod star;

use bevy::app::Plugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use crate::events::GameOver;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_event::<GameOver>()
            .add_plugins((
            EnemyPlugin,
            PlayerPlugin,
            StarPlugin,
            ScorePlugin
        ));
    }
}
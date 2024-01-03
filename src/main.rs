mod game;
mod main_menu;
mod events;
mod systems;

use bevy::prelude::*;
use events::*;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use bevy_kira_audio::prelude::*;
use systems::{handle_game_over,exit_game, spawn_camera, transition_to_game_state, transition_to_main_menu_state};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AudioPlugin))
        .add_state::<AppState>()
        .add_event::<GameOver>()
        .add_plugins((GamePlugin, MainMenuPlugin))
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (handle_game_over, exit_game))
        .add_systems(Update, (transition_to_game_state, transition_to_main_menu_state))
        .run();
}

#[derive(States, Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}


mod components;
mod systems;
mod styles;

use bevy::prelude::*;
use crate::AppState;

use self::systems::layout::{spawn_main_menu, despawn_main_menu};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}

pub fn main_menu() {
    println!("Main Menu XD");
}
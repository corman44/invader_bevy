use bevy::prelude::*;

use self::systems::{spawn_player, player_movement, confine_player_movement, enemy_hit_player, player_hit_star};

mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (
                player_movement,
                confine_player_movement.after(player_movement))
            )
            .add_systems(Update, (enemy_hit_player, player_hit_star));
    }
}
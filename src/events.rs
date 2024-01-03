use bevy::{prelude::*, app::AppExit};

#[derive(Event)]
pub struct GameOver {
    pub score: u32,
}
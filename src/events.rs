use bevy::{prelude::*, app::AppExit};

#[derive(Event)]
pub struct GameOver {
    pub score: u32,
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over( mut game_over_event_writer: EventReader<GameOver>) {
    for event in game_over_event_writer.iter() {
        println!("Final Score: {}", event.score.to_string());
    }
}
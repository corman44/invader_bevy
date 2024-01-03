use bevy::{prelude::*, app::AppExit, window::PrimaryWindow};

use crate::{events::GameOver, AppState, game::SimulationState};

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(
    mut game_over_event_writer: EventReader<GameOver>,
    mut commands: Commands,
) {
    for event in game_over_event_writer.read() {
        println!("Final Score: {}", event.score.to_string());
        commands.insert_resource(NextState(Some(AppState::GameOver)));
    }
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

pub fn transition_to_game_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if app_state.get() != &AppState::Game {
            println!("AppState: Game");
            commands.insert_resource(NextState(Some(AppState::Game)));
        }
    }
}

pub fn transition_to_main_menu_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if app_state.get() != &AppState::MainMenu {
            println!("AppState: MainMenu");
            commands.insert_resource(NextState(Some(AppState::MainMenu)));
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
        }
    }
}

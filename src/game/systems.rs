use bevy::prelude::*;

use super::SimulationState;

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("Simulation State: {:?}", simulation_state.get());
        match simulation_state.get() {
            SimulationState::Running => {
                next_sim_state.set(SimulationState::Paused);
                println!("Simulation Paused.");
            },
            SimulationState::Paused => {
                next_sim_state.set(SimulationState::Running);
                println!("Simulation Resumed.");
            },
        }
    }
}
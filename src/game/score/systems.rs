use bevy::prelude::*;

use crate::events::GameOver;
use super::resources::*;


pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

pub fn update_high_scores(
    mut game_over_event_writer: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_writer.iter() {
        high_scores.scores.push(("CJ".to_string(), event.score));
    }
}

pub fn high_scores_updated(
    high_scores: Res<HighScores>
) {
    if high_scores.is_changed() {
        println!("New High Score: {:?}", high_scores);
    }
}

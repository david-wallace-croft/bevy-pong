use super::scored::Scored;
use super::scorer::Scorer;
use ::bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
  pub player: u32,
  pub ai: u32,
}

pub fn update_score(
  mut score: ResMut<Score>,
  mut scored_event_reader: MessageReader<Scored>,
) {
  for event in scored_event_reader.read() {
    match event.0 {
      Scorer::Ai => score.ai += 1,
      Scorer::Player => score.player += 1,
    }
    // println!("Score: {} - {}", score.player, score.ai);
  }
}

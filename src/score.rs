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
  mut scored_message_reader: MessageReader<Scored>,
) {
  for message in scored_message_reader.read() {
    match message.0 {
      Scorer::Ai => score.ai += 1,
      Scorer::Player => score.player += 1,
    }
    // println!("Score: {} - {}", score.player, score.ai);
  }
}

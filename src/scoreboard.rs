use super::ai_score::AiScore;
use super::player_score::PlayerScore;
use super::score::Score;
use ::bevy::prelude::*;

pub fn update_scoreboard(
  mut player_score_text_query: Query<&mut Text, With<PlayerScore>>,
  mut ai_score_text_query: Query<
    &mut Text,
    (With<AiScore>, Without<PlayerScore>),
  >,
  score: Res<Score>,
) {
  if score.is_changed() {
    if let Ok(mut player_score_text) = player_score_text_query.single_mut() {
      player_score_text.0 = score.player.to_string();
    }

    if let Ok(mut ai_score_text) = ai_score_text_query.single_mut() {
      ai_score_text.0 = score.ai.to_string();
    }
  }
}

pub fn spawn_scoreboard(mut commands: Commands) {
  commands.spawn((
    PlayerScore,
    Text::new("0"),
    TextFont {
      font_size: FontSize::Px(72.),
      ..default()
    },
    TextColor(Color::WHITE),
    TextLayout::justify(Justify::Center),
    Node {
      position_type: PositionType::Absolute,
      top: Val::Px(5.),
      right: Val::Px(15.),
      ..default()
    },
  ));

  commands.spawn((
    AiScore,
    Text::new("0"),
    TextFont {
      font_size: FontSize::Px(72.),
      ..default()
    },
    TextColor(Color::WHITE),
    TextLayout::justify(Justify::Center),
    Node {
      position_type: PositionType::Absolute,
      top: Val::Px(5.),
      left: Val::Px(15.),
      ..default()
    },
  ));
}

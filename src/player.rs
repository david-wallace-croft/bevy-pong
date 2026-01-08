use super::velocity::Velocity;
use ::bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn handle_player_input(
  key_code_button_input: Res<ButtonInput<KeyCode>>,
  mut player_velocity_query: Query<&mut Velocity, With<Player>>,
) {
  if let Ok(mut player_velocity) = player_velocity_query.single_mut() {
    // TODO: Can two buttons be pressed simultaneously?
    if key_code_button_input.pressed(KeyCode::ArrowUp) {
      player_velocity.0.y = 1.;
    } else if key_code_button_input.pressed(KeyCode::ArrowDown) {
      player_velocity.0.y = -1.;
    } else {
      player_velocity.0.y = 0.;
    }
  }
}

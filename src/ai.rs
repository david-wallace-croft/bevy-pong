use super::ball::Ball;
use super::position::Position;
use super::velocity::Velocity;
use ::bevy::prelude::*;

#[derive(Component)]
pub struct Ai;

pub fn move_ai(
  ai_single: Single<(&mut Velocity, &Position), With<Ai>>,
  ball_position_single: Single<&Position, With<Ball>>,
) {
  let (mut velocity, position) = ai_single.into_inner();

  let a_to_b: Vec2 = ball_position_single.0 - position.0;

  velocity.0.y = a_to_b.y.signum();
}

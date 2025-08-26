use ::bevy::prelude::*;

#[derive(Component, Default)]
#[require(Transform)]
pub struct Position(pub Vec2);

pub fn project_positions(
  mut positionables: Query<(&mut Transform, &Position)>
) {
  for (mut transform, position) in &mut positionables {
    transform.translation = position.0.extend(0.);
  }
}

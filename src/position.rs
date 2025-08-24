use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Transform)]
pub(super) struct Position(pub(super) Vec2);

pub(super) fn project_positions(
  mut positionables: Query<(&mut Transform, &Position)>
) {
  for (mut transform, position) in &mut positionables {
    transform.translation = position.0.extend(0.);
  }
}

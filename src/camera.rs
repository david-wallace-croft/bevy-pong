use bevy::prelude::*;

pub(super) fn spawn_camera(mut commands: Commands) {
  println!("Spawning camera");

  commands.spawn((Camera2d, Transform::from_xyz(0., 0., 0.)));
}

use super::position::Position;
use bevy::prelude::*;

const BALL_SIZE: f32 = 5.;

#[derive(Component)]
#[require(Position)]
pub(super) struct Ball;

pub(super) fn move_ball(mut position: Single<&mut Position, With<Ball>>) {
  position.0.x += 1.0;
}

pub(super) fn spawn_ball(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  println!("Spawning ball...");

  let shape: Circle = Circle::new(BALL_SIZE);

  let color: Color = Color::srgb(1., 0., 0.);

  let mesh: Handle<Mesh> = meshes.add(shape);

  let material: Handle<ColorMaterial> = materials.add(color);

  commands.spawn((Ball, Mesh2d(mesh), MeshMaterial2d(material)));
}

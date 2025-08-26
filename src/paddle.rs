use super::position::Position;
use super::shape::Shape;
use ::bevy::prelude::*;

const PADDLE_WIDTH: f32 = 10.;

const PADDLE_HEIGHT: f32 = 50.;

#[derive(Component)]
#[require(
  Position,
  Shape = Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
)]
struct Paddle;

pub fn spawn_paddles(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
  println!("Spawning paddles...");

  let shape: Rectangle = Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT);

  let color: Color = Color::srgb(0., 1., 0.);

  let mesh_handle: Handle<Mesh> = meshes.add(shape);

  let color_material_handle: Handle<ColorMaterial> = color_materials.add(color);

  commands.spawn((
    Paddle,
    Mesh2d(mesh_handle),
    MeshMaterial2d(color_material_handle),
    Position(Vec2::new(25., 0.)),
  ));
}

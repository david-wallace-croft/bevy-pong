use super::ai::Ai;
use super::player::Player;
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
  window: Query<&Window>,
) {
  println!("Spawning paddles...");

  if let Ok(window) = window.single() {
    let window_width: f32 = window.resolution.width();

    let padding: f32 = 50.;

    let right_paddle_x: f32 = window_width / 2. - padding;

    let left_paddle_x: f32 = -window_width / 2. + padding;

    let shape: Rectangle = Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT);

    let mesh_handle: Handle<Mesh> = meshes.add(shape);

    let ai_color: Color = Color::srgb(0., 1., 0.);

    let player_color: Color = Color::srgb(0., 0., 1.);

    let ai_color_material_handle: Handle<ColorMaterial> =
      color_materials.add(ai_color);

    let player_color_material_handle: Handle<ColorMaterial> =
      color_materials.add(player_color);

    commands.spawn((
      Ai,
      Paddle,
      Position(Vec2::new(left_paddle_x, 0.)),
      Mesh2d(mesh_handle.clone()),
      MeshMaterial2d(ai_color_material_handle),
    ));

    commands.spawn((
      Player,
      Paddle,
      Position(Vec2::new(right_paddle_x, 0.)),
      Mesh2d(mesh_handle.clone()),
      MeshMaterial2d(player_color_material_handle),
    ));
  }
}

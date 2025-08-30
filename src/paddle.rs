use super::ai::Ai;
use super::constants::{
  GUTTER_HEIGHT, PADDLE_HEIGHT, PADDLE_SPEED, PADDLE_WIDTH,
};
use super::player::Player;
use super::position::Position;
use super::shape::Shape;
use super::velocity::Velocity;
use ::bevy::prelude::*;

#[derive(Component)]
#[require(
  Position,
  Shape = Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
  Velocity,
)]
pub struct Paddle;

pub fn handle_player_input(
  key_code_button_input: Res<ButtonInput<KeyCode>>,
  mut player_velocity_query: Query<&mut Velocity, With<Player>>,
) {
  if let Ok(mut player_velocity) = player_velocity_query.single_mut() {
    println!("handle_player_input");
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

pub fn move_paddles(
  mut paddle_query: Query<(&mut Position, &Velocity), With<Paddle>>,
  window: Query<&Window>,
) {
  if let Ok(window) = window.single() {
    let window_height: f32 = window.resolution.height();

    let max_y: f32 = window_height / 2. - GUTTER_HEIGHT - PADDLE_HEIGHT / 2.;

    for (mut position, velocity) in &mut paddle_query {
      let new_position: Vec2 = position.0 + velocity.0 * PADDLE_SPEED;

      if new_position.y.abs() < max_y {
        position.0 = new_position;
      }
    }
  }
}

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

use super::constants::GUTTER_HEIGHT;
use super::position::Position;
use super::shape::Shape;
use ::bevy::prelude::*;

#[derive(Component)]
#[require(
  Position, Shape
)]
pub struct Gutter;

pub fn spawn_gutters(
  mut commands: Commands,
  mut mesh_assets: ResMut<Assets<Mesh>>,
  mut color_material_assets: ResMut<Assets<ColorMaterial>>,
  window: Query<&Window>,
) {
  if let Ok(window) = window.single() {
    let window_width: f32 = window.resolution.width();

    let window_height: f32 = window.resolution.height();

    let top_gutter_y = window_height / 2. - GUTTER_HEIGHT / 2.;

    let bottom_gutter_y = -window_height / 2. + GUTTER_HEIGHT / 2.;

    let shape: Rectangle =
      Rectangle::from_size(Vec2::new(window_width, GUTTER_HEIGHT));

    let mesh_handle: Handle<Mesh> = mesh_assets.add(shape);

    let color: Color = Color::srgb(0., 0., 0.);

    let color_material_handle: Handle<ColorMaterial> =
      color_material_assets.add(color);

    commands.spawn((
      Gutter,
      Shape(shape.size()),
      Position(Vec2::new(0., top_gutter_y)),
      Mesh2d(mesh_handle.clone()),
      MeshMaterial2d(color_material_handle.clone()),
    ));

    commands.spawn((
      Gutter,
      Shape(shape.size()),
      Position(Vec2::new(0., bottom_gutter_y)),
      Mesh2d(mesh_handle),
      MeshMaterial2d(color_material_handle),
    ));
  }
}

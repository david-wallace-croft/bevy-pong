use bevy::prelude::*;

fn main() {
  let mut app: App = App::new();

  let app: &mut App = app.add_plugins(DefaultPlugins);

  let app: &mut App = app.add_systems(Startup, (spawn_ball, spawn_camera));

  let app: &mut App =
    app.add_systems(Update, (move_ball, project_positions.after(move_ball)));

  let _app_exit: AppExit = app.run();
}

const BALL_SIZE: f32 = 5.;

fn spawn_ball(
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

fn spawn_camera(mut commands: Commands) {
  println!("Spawning camera");

  commands.spawn((Camera2d, Transform::from_xyz(0., 0., 0.)));
}

#[derive(Component, Default)]
#[require(Transform)]
struct Position(Vec2);

#[derive(Component)]
#[require(Position)]
struct Ball;

fn move_ball(mut position: Single<&mut Position, With<Ball>>) {
  position.0.x += 1.0;
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
  for (mut transform, position) in &mut positionables {
    transform.translation = position.0.extend(0.);
  }
}
